//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1564;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1565;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta306<F: Float>(t240: F, t2719: F, t243: F, t2722: F, t2723: F, t2661: F, t231: F, t2662: F, t10489: F, t828: F, t855: F, t221: F, t2430: F, t2675: F, t2674: F, t2735: F, t2783: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t10726 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1564::<F>(t240, t2719);
        let (t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1565::<F>(t243, t2722, t2723, t10726, t2661, t231, t2662, t10489, t828, t855, t221, t2430, t2675);
        let (t10742, t10744) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1566::<F>(t10741, t2674, t2735, t2783);
    (t10726, t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741, t10742, t10744)
}
