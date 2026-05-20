//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1799;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta509<F: Float>(t25924: F, t30278: F, t1903: F, t8085: F, t7296: F, t1904: F, t213: F, t25930: F, t26238: F, t26251: F, t26263: F, t26279: F, t26294: F, t27837: F, t28781: F, t28783: F, t28796: F, t28899: F, t30227: F, t30248: F, t30252: F, t30257: F, t30262: F, t30267: F, t561: F, t6896: F, t7295: F, t7511: F, t8100: F, t30247: F, t545: F, t2028: F, t2097: F, t6918: F, t2027: F, t2103: F, t26309: F, t26361: F, t26363: F, t26365: F, t28826: F, t28838: F, t28846: F, t28853: F, t28858: F, t28895: F, t28897: F, t28903: F, t28909: F, t30071: F, t6919: F, t7917: F, t8095: F, t8104: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t30279, t30282, t30283, t30286) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1799::<F>(t25924, t30278, t1903, t8085, t7296, t1904, t213, t25930, t26238, t26251, t26263, t26279, t26294, t27837, t28781, t28783, t28796, t28899, t30227, t30248, t30252, t30257, t30262, t30267, t561, t6896, t7295, t7511, t8100);
        let (t30295, t30296, t30308, t30309, t30312) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1800::<F>(t30247, t545, t2028, t2097, t6918, t7296, t2027, t2103, t26309, t26361, t26363, t26365, t27837, t28826, t28838, t28846, t28853, t28858, t28895, t28897, t28903, t28909, t30071, t6919, t7295, t7511, t7917, t8095, t8104);
    (t30279, t30282, t30283, t30286, t30295, t30296, t30308, t30309, t30312)
}
