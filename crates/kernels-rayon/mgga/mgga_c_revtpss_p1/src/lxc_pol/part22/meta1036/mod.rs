//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1036 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1036(t16682: f64, t5192: f64, t20652: f64, t44012: f64, t12227: f64, t20651: f64, t3427: f64, t3385: f64, t44091: f64, t44093: f64, t6438: f64, t5219: f64, t5412: f64, t16750: f64, t1774: f64, t1211: f64, t1215: f64, t12658: f64, t1274: f64, t1770: f64, t17963: f64, t17979: f64, t17986: f64, t17991: f64, t18065: f64, t18087: f64, t1828: f64, t21389: f64, t21618: f64, t21624: f64, t3556: f64, t3567: f64, t3575: f64, t3732: f64, t3736: f64, t3737: f64, t45482: f64, t5498: f64, t6574: f64, t6580: f64, t6744: f64, t1284: f64, t21333: f64, t68243: f64, t68245: f64, t68247: f64, t68250: f64, t68602: f64, t68604: f64, t68608: f64, t68611: f64, t68613: f64, t68621: f64, t68625: f64, t68628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68631, t68633, t68636, t68640, t68658) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623(t16682, t5192, t20652, t44012, t12227, t20651, t3427, t3385, t44091, t44093, t6438, t5219, t5412);
        let (t68669, t68673) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624(t16750, t1774, t1211, t1215, t12658, t1274, t1770, t17963, t17979, t17986, t17991, t18065, t18087, t1828, t21389, t21618, t21624, t3556, t3567, t3575, t3732, t3736, t3737, t45482, t5498, t6574, t6580, t6744, t68658);
        let (t68674, t68679) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625(t1284, t21333, t68243, t68245, t68247, t68250, t68602, t68604, t68608, t68611, t68613, t68621, t68625, t68628);
    (t68631, t68633, t68636, t68640, t68669, t68673, t68674, t68679)
}
