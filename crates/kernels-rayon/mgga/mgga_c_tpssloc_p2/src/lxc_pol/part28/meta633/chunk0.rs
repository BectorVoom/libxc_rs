//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2001/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001(t1081: f64, t1649: f64, t1877: f64, t2057: f64, t23789: f64, t23813: f64, t24191: f64, t24335: f64, t2522: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t3231: f64, t4314: f64, t47645: f64, t7114: f64, t7649: f64, t7845: f64, t7871: f64, t89859: f64, t89862: f64, t89865: f64, t89868: f64, t89874: f64, t89896: f64, t89904: f64, t89954: f64, t92319: f64) -> f64 {
    let t93181 = 3.0_f64 / 2.0_f64 * t2522 * t24335 * t7649 - t1877 * t26744 * t23813 / 2.0_f64 + 3.0_f64 * t47645 * t7871 - 3.0_f64 * t26563 * t89865 - 3.0_f64 * t26756 * t89954 + t1877 * t26740 * t1081 + 6.0_f64 * t26563 * t89896 + t1877 * t24335 * t1649 / 2.0_f64 - 3.0_f64 * t24191 * t89862 - 3.0_f64 * t92319 * t23789 + 6.0_f64 * t26563 * t89859 - t1877 * t7114 * t89868 / 2.0_f64 + t1877 * t7845 * t3231 / 2.0_f64 + 3.0_f64 * t4314 * t2057 * t89874 + 3.0_f64 * t24191 * t89904;
    t93181
}
