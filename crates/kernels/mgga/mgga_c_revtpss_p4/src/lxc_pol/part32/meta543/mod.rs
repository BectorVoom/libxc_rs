//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1854;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta543<F: Float>(t26481: F, t93182: F, t25411: F, t136: F, t2457: F, t7423: F, t25299: F, t25431: F, t26555: F, t40270: F, t25305: F, t25410: F, t7419: F, t93240: F, t26519: F, t93160: F, t25372: F, t95536: F, t7398: F, t822: F, t93170: F, t95746: F, t7064: F, t95575: F, t2067: F, t41117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95786, t95794, t95796, t95807, t95808, t95811) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1854::<F>(t26481, t93182, t25411, t136, t2457, t7423, t25299, t25431, t26555, t40270, t25305, t25410, t7419, t93240);
        let (t95813, t95822, t95825, t95836, t95859, t95862) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855::<F>(t26519, t93160, t25372, t95536, t7398, t822, t93170, t95746, t7064, t95575, t2067, t41117);
    (t95786, t95794, t95796, t95807, t95808, t95811, t95813, t95822, t95825, t95836, t95859, t95862)
}
