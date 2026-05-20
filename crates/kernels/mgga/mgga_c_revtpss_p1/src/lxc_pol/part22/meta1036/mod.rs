//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1036 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1036<F: Float>(t16682: F, t5192: F, t20652: F, t44012: F, t12227: F, t20651: F, t3427: F, t3385: F, t44091: F, t44093: F, t6438: F, t5219: F, t5412: F, t16750: F, t1774: F, t1211: F, t1215: F, t12658: F, t1274: F, t1770: F, t17963: F, t17979: F, t17986: F, t17991: F, t18065: F, t18087: F, t1828: F, t21389: F, t21618: F, t21624: F, t3556: F, t3567: F, t3575: F, t3732: F, t3736: F, t3737: F, t45482: F, t5498: F, t6574: F, t6580: F, t6744: F, t1284: F, t21333: F, t68243: F, t68245: F, t68247: F, t68250: F, t68602: F, t68604: F, t68608: F, t68611: F, t68613: F, t68621: F, t68625: F, t68628: F) -> (F, F, F, F, F, F, F, F) {
        let (t68631, t68633, t68636, t68640, t68658) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3623::<F>(t16682, t5192, t20652, t44012, t12227, t20651, t3427, t3385, t44091, t44093, t6438, t5219, t5412);
        let (t68669, t68673) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624::<F>(t16750, t1774, t1211, t1215, t12658, t1274, t1770, t17963, t17979, t17986, t17991, t18065, t18087, t1828, t21389, t21618, t21624, t3556, t3567, t3575, t3732, t3736, t3737, t45482, t5498, t6574, t6580, t6744, t68658);
        let (t68674, t68679) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625::<F>(t1284, t21333, t68243, t68245, t68247, t68250, t68602, t68604, t68608, t68611, t68613, t68621, t68625, t68628);
    (t68631, t68633, t68636, t68640, t68669, t68673, t68674, t68679)
}
