//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1240/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1240(t19577: f64, t22574: f64, t36363: f64, t24995: f64, t37589: f64, t5308: f64, t1983: f64, t31221: f64, t5161: f64, t120063: f64, t120064: f64, t120067: f64, t120069: f64, t120072: f64, t120075: f64, t120078: f64, t120079: f64, t120083: f64, t120085: f64, t120086: f64, t120088: f64, t31055: f64, t31057: f64, t31060: f64) -> f64 {
    let t120092 = 3.0_f64 * t22574 * t36363 * t19577;
    let t120095 = 6.0_f64 * t24995 * t37589 * t5308;
    let t120097 = t1983 * t31221 * t5161;
    let t120098 = -t31055 - t31057 - t31060 - t120063 - 4.0_f64 * t120064 - t120067 - t120069 + 2.0_f64 * t120072 - t120075 + t120078 + 6.0_f64 * t120079 - t120083 + t120085 + 6.0_f64 * t120086 + 6.0_f64 * t120088 - t120092 + t120095 - t120097;
    t120098
}
