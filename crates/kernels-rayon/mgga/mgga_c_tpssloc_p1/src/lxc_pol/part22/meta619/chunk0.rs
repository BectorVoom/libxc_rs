//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2150/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2150(t11708: f64, t15502: f64, t15506: f64, t10469: f64, t1720: f64, t10471: f64, t11737: f64, t11791: f64, t5005: f64, t11677: f64, t15027: f64, t3575: f64, t373: f64, t470: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52810 = t11708 * t15502;
    let t52813 = t11708 * t15506;
    let t52834 = t1720 * t10469;
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    let t52872 = t5005 * t11791;
    let t52873 = t52872 / 6912.0_f64;
    let t52879 = t15027 * t11677;
    let t52893 = t470 * t493 * t3575 * t373;
    (t52810, t52813, t52834, t52835, t52836, t52873, t52879, t52893)
}
