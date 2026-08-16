//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1222/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1222(t1081: f64, t1877: f64, t2057: f64, t2068: f64, t23781: f64, t23789: f64, t23796: f64, t23807: f64, t24191: f64, t24335: f64, t2522: f64, t26563: f64, t26756: f64, t3231: f64, t4314: f64, t6841: f64, t7110: f64, t7114: f64, t82320: f64, t83559: f64, t83585: f64, t83592: f64, t83596: f64, t83624: f64, t83645: f64, t83651: f64, t84766: f64, t84797: f64, t84800: f64) -> f64 {
    let t85337 = 3.0_f64 * t26756 * t83645 + 3.0_f64 * t82320 * t2068 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t83596 + 9.0_f64 / 2.0_f64 * t2522 * t7110 * t23796 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t83592 + 9.0_f64 / 2.0_f64 * t2522 * t24335 * t6841 + 3.0_f64 * t1877 * t84800 * t23807 + 3.0_f64 / 2.0_f64 * t1877 * t7110 * t3231 - 9.0_f64 * t84797 * t23789 + 9.0_f64 * t4314 * t7110 * t23781 - 3.0_f64 * t1877 * t84766 * t83585 - 9.0_f64 / 2.0_f64 * t24191 * t83651 - t1877 * t7114 * t83559 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t1877 * t24335 * t1081 - 9.0_f64 * t26563 * t83624;
    t85337
}
