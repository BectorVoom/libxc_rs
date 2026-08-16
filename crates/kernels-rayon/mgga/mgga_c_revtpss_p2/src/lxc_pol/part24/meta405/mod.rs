//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1343;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta405(t16: f64, t2236: f64, t240: f64, t236: f64, t243: f64, t281: f64, t39644: f64, t10696: f64, t72: f64, t245: f64, t10697: f64, t136: f64, t2452: f64, t9720: f64, t225: f64, t268: f64, t10868: f64, t2237: f64, t2482: f64, t849: f64, t234: f64, t9801: f64, t2475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40649, t40650, t40654, t40673, t40683) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1343(t16, t2236, t240, t236, t243, t281, t39644, t10696, t72, t245, t10697, t136);
        let (t40688, t40689, t40690, t40693, t40710, t40721, t40724) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1344(t2452, t9720, t225, t268, t10868, t240, t2237, t2482, t849, t234, t9801, t136, t2475);
    (t40649, t40650, t40654, t40673, t40683, t40688, t40689, t40690, t40693, t40710, t40721, t40724)
}
