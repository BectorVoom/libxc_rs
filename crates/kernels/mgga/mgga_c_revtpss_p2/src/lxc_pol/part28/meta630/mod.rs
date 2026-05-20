//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2272;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2273;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta630<F: Float>(t4158: F, t7950: F, t18190: F, t2042: F, t1459: F, t28271: F, t5795: F, t7334: F, t1518: F, t572: F, t95137: F, t26123: F, t4292: F, t101613: F, t101617: F, t101619: F, t101621: F, t101625: F, t101628: F, t1461: F, t18211: F, t2040: F, t28246: F, t4162: F, t4165: F, t5802: F, t5805: F, t7324: F, t7944: F, t28283: F, t571: F, t28234: F, t575: F, t101558: F, t101563: F, t101609: F, t1456: F, t1458: F, t1914: F, t1921: F, t26094: F, t26133: F, t3: F, t4168: F, t5808: F, t7319: F, t7940: F, t92559: F, t92563: F, t95127: F, t1455: F, t7956: F, t1464: F, t7939: F, t2037: F, t7318: F, t2045: F, t5789: F, t18178: F, t18217: F, t2038: F, t28235: F, t4154: F, t5790: F, t7337: F, t92556: F, t95125: F, t95180: F) -> F {
        let (t101632, t101634, t101640, t101642, t101645, t101648) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2272::<F>(t4158, t7950, t18190, t2042, t1459, t28271, t5795, t7334, t1518, t572, t95137, t26123, t4292);
        let t101651 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2273::<F>(t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648, t1461, t18211, t2040, t28246, t4162, t4165, t5802, t5805, t7324, t7944);
        let t101659 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274::<F>(t28283, t571, t28234, t575, t101558, t101563, t101609, t101651, t1456, t1458, t1914, t1921, t26094, t26133, t3, t4168, t5808, t7319, t7940, t92559, t92563, t95127);
        let t101678 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275::<F>(t1455, t7956, t1464, t7939, t2037, t5808, t1921, t7318, t2045, t5789, t18178, t18217, t2038, t28235, t4154, t5790, t7337, t92556, t95125, t95180);
        let tv4rho3sigma3 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2276::<F>(t101659, t101678);
    tv4rho3sigma3
}
