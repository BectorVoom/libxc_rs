//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1752;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta572<F: Float>(t1196: F, t12552: F, t3523: F, t90357: F, t12248: F, t6470: F, t6474: F, t1732: F, t24324: F, t3384: F, t3433: F, t81650: F, t12227: F, t20651: F, t1765: F, t82389: F, t20400: F, t6552: F, t12254: F, t141: F, t89863: F, t1145: F, t89845: F, t89853: F, t89822: F, t68255: F, t68257: F, t81156: F, t81158: F, t89839: F, t89851: F, t89865: F, t89869: F, t89873: F, t89877: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90361, t90364, t90367, t90370) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751::<F>(t1196, t12552, t3523, t90357, t12248, t6470, t6474, t1732, t24324, t3384, t3433, t81650);
        let (t90373, t90375, t90377, t90379, t90384) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1752::<F>(t12227, t20651, t6470, t1765, t82389, t20400, t6552, t12254, t141, t89863, t1145, t89845);
        let (t90387, t90390, t90400) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753::<F>(t1145, t141, t89853, t12254, t89822, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384);
    (t90361, t90364, t90367, t90370, t90373, t90375, t90377, t90379, t90384, t90387, t90390, t90400)
}
