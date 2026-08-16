//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2344;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2345;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2346;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta639(t10963: f64, t9303: f64, t2434: f64, t2626: f64, t2629: f64, t676: f64, t9425: f64, t2567: f64, t2576: f64, t2582: f64, t2577: f64, t268: f64, t9326: f64, t215: f64, t2581: f64, t2585: f64, t675: f64, t9273: f64, t9276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39724, t39739, t39741, t39742, t39744, t39747) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2344(t10963, t9303, t2434, t2626, t2629, t676, t9425, t2567, t2576, t2582);
        let t39750 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2345(t2577, t268, t9326);
        let t39756 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2346(t215, t2581, t2585, t268);
        let t39760 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2347(t268, t675, t9273, t9276);
    (t39724, t39739, t39741, t39742, t39744, t39747, t39750, t39756, t39760)
}
