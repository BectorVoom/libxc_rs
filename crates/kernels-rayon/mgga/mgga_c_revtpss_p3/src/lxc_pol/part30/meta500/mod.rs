//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1865;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1866;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta500(t1248: f64, t7644: f64, t1287: f64, t3588: f64, t7660: f64, t11239: f64, t487: f64, t1276: f64, t2148: f64, t2142: f64, t3596: f64, t3601: f64, t3769: f64, t3783: f64, t1269: f64, t3140: f64, t1243: f64, t8939: f64, t2149: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26897, t26901, t26906, t26909) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1865(t1248, t7644, t1287, t3588, t7660, t11239, t487, t1276, t2148, t2142, t3596, t3601, t3769);
        let (t26913, t26918, t26921) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1866(t3601, t3783, t7660, t1269, t3140, t1276, t2148, t1243, t8939);
        let t26922 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1867(t2149, t26921);
    (t26897, t26901, t26906, t26909, t26913, t26918, t26921, t26922)
}
