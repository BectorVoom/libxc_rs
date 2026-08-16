//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1302/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1302(t26982: f64, t3565: f64, t7635: f64, t1032: f64, t3727: f64, t2148: f64, t11239: f64, t1269: f64, t1276: f64, t42859: f64, t487: f64, t13038: f64, t2142: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96870 = t26982 * t3565 * t7635;
    let t96873 = t3727 * t1032;
    let t96874 = t2148 * t96873;
    let t96881 = t1269 * t11239;
    let t96883 = t2148 * t96881 * t1276;
    let t96886 = t487 * t42859;
    let t96888 = t2148 * t96886 * t1276;
    let t96889 = t13038 * t2142;
    (t96870, t96873, t96874, t96883, t96888, t96889)
}
