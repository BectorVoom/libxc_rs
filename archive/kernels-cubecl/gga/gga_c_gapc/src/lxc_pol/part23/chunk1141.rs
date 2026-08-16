//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1141/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1141<F: Float>(t11522: F, t15805: F, t9799: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34125: F, t34127: F, t34132: F) -> F {
    let t34135 = t15805 * t11522 * t9799;
    let t34137 = F::cast_from(0.2318836277704281739e-4_f64) * t34104 + F::cast_from(0.56360603971979070047e-7_f64) * t34108 + F::cast_from(0.34752370105806885418e-3_f64) * t34111 - F::cast_from(0.24581606547037760418e-7_f64) * t34114 + F::cast_from(0.12290803273518880209e-8_f64) * t34117 - F::cast_from(0.35170937063461460536e-8_f64) * t34119 - F::cast_from(0.35170937063461460536e-8_f64) * t34121 + F::cast_from(0.4797801045921060808e-7_f64) * t34125 + F::cast_from(0.17089546493091976008e-5_f64) * t34127 - F::cast_from(0.12290803273518880209e-8_f64) * t34132 + F::cast_from(0.12650553385416666667e-5_f64) * t34135;
    t34137
}
