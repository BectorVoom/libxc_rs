//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2385;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta624(t2661: f64, t2662: f64, t2749: f64, t40378: f64, t2430: f64, t853: f64, t837: f64, t836: f64, t124: f64, t2645: f64, t14686: f64, t14931: f64, t4366: f64, t2722: f64, t10777: f64, t10779: f64, t2682: f64, t820: f64, t823: f64, t2751: f64, t10886: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40553, t40555, t40558, t40560, t40578, t40581) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384(t2661, t2662, t2749, t40378, t2430, t853, t837, t836, t124, t2645, t14686, t14931, t4366);
        let (t40583, t40586, t40593, t40594, t40600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2385(t124, t2722, t10777, t10779, t2749, t2682, t820, t823, t2751, t10886, t40555, t808);
    (t40553, t40558, t40560, t40578, t40581, t40583, t40586, t40593, t40594, t40600)
}
