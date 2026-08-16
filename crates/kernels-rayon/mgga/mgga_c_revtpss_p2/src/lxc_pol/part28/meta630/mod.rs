//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2272;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2273;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta630(t4158: f64, t7950: f64, t18190: f64, t2042: f64, t1459: f64, t28271: f64, t5795: f64, t7334: f64, t1518: f64, t572: f64, t95137: f64, t26123: f64, t4292: f64, t101613: f64, t101617: f64, t101619: f64, t101621: f64, t101625: f64, t101628: f64, t1461: f64, t18211: f64, t2040: f64, t28246: f64, t4162: f64, t4165: f64, t5802: f64, t5805: f64, t7324: f64, t7944: f64, t28283: f64, t571: f64, t28234: f64, t575: f64, t101558: f64, t101563: f64, t101609: f64, t1456: f64, t1458: f64, t1914: f64, t1921: f64, t26094: f64, t26133: f64, t3: f64, t4168: f64, t5808: f64, t7319: f64, t7940: f64, t92559: f64, t92563: f64, t95127: f64, t1455: f64, t7956: f64, t1464: f64, t7939: f64, t2037: f64, t7318: f64, t2045: f64, t5789: f64, t18178: f64, t18217: f64, t2038: f64, t28235: f64, t4154: f64, t5790: f64, t7337: f64, t92556: f64, t95125: f64, t95180: f64) -> f64 {
        let (t101632, t101634, t101640, t101642, t101645, t101648) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2272(t4158, t7950, t18190, t2042, t1459, t28271, t5795, t7334, t1518, t572, t95137, t26123, t4292);
        let t101651 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2273(t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648, t1461, t18211, t2040, t28246, t4162, t4165, t5802, t5805, t7324, t7944);
        let t101659 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2274(t28283, t571, t28234, t575, t101558, t101563, t101609, t101651, t1456, t1458, t1914, t1921, t26094, t26133, t3, t4168, t5808, t7319, t7940, t92559, t92563, t95127);
        let t101678 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275(t1455, t7956, t1464, t7939, t2037, t5808, t1921, t7318, t2045, t5789, t18178, t18217, t2038, t28235, t4154, t5790, t7337, t92556, t95125, t95180);
        let tv4rho3sigma3 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2276(t101659, t101678);
    tv4rho3sigma3
}
