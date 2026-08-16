//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta523<F: Float>(t5876: F, t670: F, t1448: F, t6836: F, t6816: F, t1868: F, t5778: F, t10309: F, t607: F, t2411: F, t605: F, t1955: F, t25308: F, t2769: F, t7036: F, t820: F, t844: F, t2482: F, t814: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F, t228: F, t25273: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85360, t86753, t86771, t86815, t92568, t92790, t92917) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826::<F>(t5876, t670, t1448, t6836, t6816, t1868, t5778, t10309, t607, t2411, t605, t1955, t25308, t2769);
        let (t92951, t92955, t92963, t92966, t92968) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1827::<F>(t7036, t820, t844, t2482, t814, t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273);
    (t85360, t86753, t86771, t86815, t92568, t92790, t92917, t92951, t92955, t92963, t92966, t92968)
}
