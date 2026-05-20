//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1053;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta297<F: Float>(t2664: F, t9794: F, t10760: F, t2475: F, t72: F, t245: F, t2482: F, t814: F, t823: F, t136: F, t853: F, t220: F, t124: F, t836: F, t2749: F, t820: F, t844: F, t2751: F, t2681: F, t839: F, t222: F, t9727: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10762, t10769, t10770, t10777, t10779) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1053::<F>(t2664, t9794, t10760, t2475, t72, t245, t2482, t814, t823, t136, t853, t220);
        let (t10783, t10811, t10812, t10815, t10816, t10824) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1054::<F>(t124, t836, t10779, t2749, t10777, t820, t823, t844, t2751, t2681, t839, t222, t9727);
    (t10762, t10769, t10770, t10777, t10779, t10783, t10811, t10812, t10815, t10816, t10824)
}
