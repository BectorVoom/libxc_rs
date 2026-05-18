//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 284/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk284<F: Float>(t11: F, t19: F, t1101: F, t1108: F, t350: F, t21: F, t405: F, t1112: F, t1114: F, t1116: F, t356: F, t340: F) -> (F, F, F, F) {
    let t1142 = F::new(1.0)/f64::sqrt(t11);
    let t1143 = t1142 * t19;
    let t1144 = t1143 * t1101;
    let t1146 = t350 * t1108;
    let t1148 = t21 * t405;
    let t1150 = -F::new(0.42198333333333333333e0) * t1112 + F::new(0.84396666666666666666e0) * t1114 + F::new(0.39862222222222222223e0) * t1116 + F::new(0.68258333333333333333e-1) * t1144 + F::new(0.13651666666666666667e0) * t1146 + F::new(0.13692777777777777778e0) * t1148;
    let t1151 = t1150 * t356;
    let t1153 = F::new(1.0) * t340 * t1151;
    (t1144, t1146, t1148, t1153)
}
