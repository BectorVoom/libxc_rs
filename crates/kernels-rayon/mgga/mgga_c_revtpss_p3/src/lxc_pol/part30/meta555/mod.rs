//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta555(t644: f64, t6977: f64, t1927: f64, t2315: f64, t2247: f64, t2259: f64, t843: f64, t1962: f64, t41154: f64, t2411: f64, t25435: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t92576, t92584, t92588, t92612, t92742, t92775, t92790) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1994(t644, t6977, t1927, t2315, t2247, t2259, t843, t1962, t41154, t2411, t25435, t605);
    (t92576, t92584, t92588, t92612, t92742, t92775, t92790)
}
