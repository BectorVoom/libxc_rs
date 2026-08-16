//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 847/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk847(t1676: f64, t3501: f64, t1535: f64, t5025: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5186: f64, t5324: f64, t5333: f64, t5338: f64, t5344: f64, t568: f64, t8845: f64, t8846: f64, t8848: f64, t8849: f64, t8851: f64, t8853: f64, t8854: f64, t8855: f64) -> (f64, f64) {
    let t9121 = t3501 * t1676;
    let t9125 = -3.0_f64 * t1535 * t568 * t9121 + t5025 + t5028 + t5040 + t5066 - t5069 - t5073 + t5186 - t5324 + t5333 - t5338 - t5344 + t8845 + t8846 + t8848 + t8849 + t8851 - t8853 + t8854 - t8855;
    (t9121, t9125)
}
