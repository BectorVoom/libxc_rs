//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 284/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk284<F: Float>(t343: F, t55: F, t97: F, t360: F, t4: F, t44: F, t375: F, t79: F, t1112: F, t1114: F, t1116: F, t1144: F, t1146: F, t1148: F) -> (F, F, F, F, F) {
    let t1163 = t343 * t97 * t55;
    let t1165 = F::cast_from(0.24415406715670879921e-3_f64) * t360 * t1163;
    let t1166 = t44 * t4;
    let t1167 = t79 * t375;
    let t1169 = F::cast_from(0.10843580882781524214e-1_f64) * t1166 * t1167;
    let t1176 = -F::cast_from(0.57538888888888888889e0_f64) * t1112 + F::cast_from(0.11507777777777777778e1_f64) * t1114 + F::cast_from(0.40256666666666666667e0_f64) * t1116 + F::new(0.366775e-1) * t1144 + F::new(0.73355e-1) * t1146 + F::new(0.137975e0) * t1148;
    (t1163, t1165, t1167, t1169, t1176)
}
