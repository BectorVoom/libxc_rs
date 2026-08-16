//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk506;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk507;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk508;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk509;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk510;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk511;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta86(t186: f64, t215: f64, t685: f64, t755: f64, t2492: f64, t2596: f64, t745: f64, t760: f64, t123: f64, t192: f64, t676: f64, t762: f64, t73: f64, t853: f64, t820: f64, t843: f64, t849: f64, t212: f64, t27: f64, t225: f64, t816: f64, t240: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2619 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk506(t186, t215, t685);
        let (t2621, t2626) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk507(t2619, t755, t2492, t2596, t745);
        let (t2628, t2629) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk508(t2626, t760, t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk509(t676, t762);
        let (t2632, t2638, t2652) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk510(t2629, t2630, t73, t853, t820, t843, t849);
        let t2661 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk511(t212, t27, t225, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk512(t240, t823);
    (t2619, t2621, t2626, t2628, t2629, t2630, t2632, t2638, t2652, t2661, t2662)
}
