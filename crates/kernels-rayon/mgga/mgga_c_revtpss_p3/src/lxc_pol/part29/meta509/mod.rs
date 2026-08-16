//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1827;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta509(t1927: f64, t2315: f64, t2247: f64, t2259: f64, t2411: f64, t605: f64, t268: f64, t41040: f64, t837: f64, t1032: f64, t2760: f64, t867: f64, t7063: f64, t1955: f64, t25308: f64, t2769: f64, t7036: f64, t820: f64, t844: f64, t2751: f64, t2482: f64, t814: f64, t10782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92584, t92588, t92790, t92840, t92888, t92889) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1827(t1927, t2315, t2247, t2259, t2411, t605, t268, t41040, t837, t1032, t2760, t867);
        let (t92890, t92917, t92951, t92952, t92955, t92956) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1828(t7063, t92889, t1955, t25308, t2769, t7036, t820, t844, t2751, t2482, t814, t10782);
    (t92584, t92588, t92790, t92840, t92888, t92889, t92890, t92917, t92951, t92952, t92955, t92956)
}
