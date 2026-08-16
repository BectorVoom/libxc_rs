//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1186/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1186(t3610: f64, t52627: f64, t1227: f64, t1653: f64, t248: f64, t45293: f64, t11677: f64, t15245: f64, t10469: f64, t1720: f64, t10471: f64, t11737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52628 = t3610 * t52627;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52766 = t15245 * t11677;
    let t52834 = t1720 * t10469;
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    (t52628, t52680, t52766, t52834, t52835, t52836)
}
