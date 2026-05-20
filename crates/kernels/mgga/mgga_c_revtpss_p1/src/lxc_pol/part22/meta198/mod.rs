//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1248;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1249;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1250;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1251;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1252;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1253;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1254;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1255;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta198<F: Float>(t4732: F, t4733: F, t981: F, t2848: F, t3037: F, t4571: F, t4576: F, t4581: F, t4585: F, t341: F, t1646: F, t993: F, t378: F, t1647: F, t1651: F, t999: F, t996: F, t1096: F, t1079: F, t3070: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4734, t4736, t4742, t4743) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1248::<F>(t4732, t4733, t981, t2848, t3037, t4571, t4576, t4581, t4585, t341);
        let t4746 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1249::<F>(t1646, t993);
        let t4747 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1250::<F>(t378, t4746);
        let t4752 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1251::<F>(t1647, t378);
        let t4757 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1252::<F>(t1651, t999);
        let t4758 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1253::<F>(t4757, t996);
        let t4764 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1254::<F>(t1096, t1651, t1079);
        let t4772 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1255::<F>(t2848, t3070, t4571, t4576, t4581, t4585);
        let t4773 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1256::<F>(t4772, t996);
    (t4734, t4736, t4742, t4743, t4746, t4747, t4752, t4757, t4758, t4764, t4772, t4773)
}
