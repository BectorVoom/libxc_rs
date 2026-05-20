//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1782;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1783;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta499<F: Float>(t14230: F, t28911: F, t26304: F, t27972: F, t27864: F, t1445: F, t1904: F, t2027: F, t213: F, t25930: F, t26282: F, t26365: F, t26366: F, t27868: F, t28863: F, t28890: F, t28895: F, t28897: F, t28899: F, t28903: F, t28905: F, t28909: F, t561: F, t5775: F, t7295: F, t7511: F, t28799: F, t28822: F, t28861: F, t532: F, t1450: F, t5627: F, t9069: F, t26411: F, t7900: F, t28176: F, t7488: F, t531: F, t8107: F, t7238: F, t2014: F, t2056: F, t2093: F, t2108: F, t27123: F, t27126: F, t27833: F, t28167: F, t28760: F, t4248: F, t5787: F, t651: F, t7235: F, t7367: F, t7374: F, t7489: F, t7732: F, t7898: F, t8079: F, t8109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28912, t28915, t28918, t28923) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1782::<F>(t14230, t28911, t26304, t27972, t27864, t1445, t1904, t2027, t213, t25930, t26282, t26365, t26366, t27868, t28863, t28890, t28895, t28897, t28899, t28903, t28905, t28909, t561, t5775, t7295, t7511);
        let (t28925, t28926, t28927, t28929, t28932, t28935) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1783::<F>(t28799, t28822, t28861, t28923, t532, t1450, t5627, t9069, t26411, t7900, t28176, t7488);
        let (t28938, t28939, t28942) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1784::<F>(t531, t8107, t7238, t2014, t2056, t2093, t2108, t27123, t27126, t27833, t28167, t28760, t28927, t28929, t28932, t28935, t4248, t5787, t651, t7235, t7367, t7374, t7489, t7732, t7898, t8079, t8109);
    (t28912, t28915, t28918, t28925, t28926, t28927, t28929, t28932, t28935, t28938, t28939, t28942)
}
