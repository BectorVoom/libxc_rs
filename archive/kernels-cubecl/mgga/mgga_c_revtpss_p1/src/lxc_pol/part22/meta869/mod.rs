//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta869 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta869<F: Float>(t14923: F, t14927: F, t10811: F, t14697: F, t40672: F, t828: F, t10905: F, t14825: F, t14829: F, t14819: F, t40517: F, t14910: F, t4423: F, t836: F, t14741: F, t2710: F, t2713: F, t10744: F, t14861: F, t808: F, t40791: F, t4442: F, t14468: F, t236: F, t807: F, t854: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51000, t51006, t51014, t51026, t51028, t51042, t51047) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027::<F>(t14923, t14927, t10811, t14697, t40672, t828, t10905, t14825, t14829, t14819, t40517, t14910);
        let (t51049, t51055, t51058, t51060, t51070) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3028::<F>(t4423, t836, t14741, t2710, t2713, t10744, t14861, t808, t40791, t4442, t14468, t236, t807, t854);
    (t51000, t51006, t51014, t51026, t51028, t51042, t51047, t51049, t51055, t51058, t51060, t51070)
}
