//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2401/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2401(t1828: f64, t3568: f64, t1277: f64, t1294: f64, t5497: f64, t3737: f64, t17288: f64, t487: f64) -> (f64, f64, f64, f64, f64) {
    let t18102 = t1828 * t3568;
    let t18103 = t1277 * t18102;
    let t18108 = t5497 * t1294;
    let t18109 = t3737 * t18108;
    let t18114 = t17288 * t487;
    (t18102, t18103, t18108, t18109, t18114)
}
