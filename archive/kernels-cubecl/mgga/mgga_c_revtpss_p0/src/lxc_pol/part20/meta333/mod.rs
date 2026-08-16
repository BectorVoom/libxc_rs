//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1251;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta333<F: Float>(t13128: F, t13143: F, t11239: F, t13038: F, t460: F, t12051: F, t13045: F, t13111: F, t3783: F, t3568: F, t3759: F, t12629: F, t1280: F, t1204: F, t1234: F, t12769: F, t1281: F, t1285: F, t12966: F, t12975: F, t12987: F, t13108: F, t13112: F, t13118: F, t13121: F, t13127: F, t13130: F, t13134: F, t13142: F, t3666: F, t3670: F, t3746: F, t3751: F, t3760: F, t3763: F, t3767: F, t3778: F, t3782: F, t3787: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13144, t13147, t13148, t13149, t13150, t13153, t13156, t13161) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1251::<F>(t13128, t13143, t11239, t13038, t460, t12051, t13045, t13111, t3783, t3568, t3759, t12629, t1280);
        let t13164 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1252::<F>(t1204, t1234, t12769, t1281, t1285, t12966, t12975, t12987, t13108, t13112, t13118, t13121, t13127, t13130, t13134, t13142, t13144, t13148, t13150, t13153, t13156, t13161, t3666, t3670, t3746, t3751, t3760, t3763, t3767, t3778, t3782, t3787, t460);
    (t13144, t13147, t13148, t13149, t13150, t13153, t13156, t13161, t13164)
}
