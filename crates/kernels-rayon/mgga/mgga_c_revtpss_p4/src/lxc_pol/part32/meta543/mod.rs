//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1854;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta543(t26481: f64, t93182: f64, t25411: f64, t136: f64, t2457: f64, t7423: f64, t25299: f64, t25431: f64, t26555: f64, t40270: f64, t25305: f64, t25410: f64, t7419: f64, t93240: f64, t26519: f64, t93160: f64, t25372: f64, t95536: f64, t7398: f64, t822: f64, t93170: f64, t95746: f64, t7064: f64, t95575: f64, t2067: f64, t41117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95786, t95794, t95796, t95807, t95808, t95811) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1854(t26481, t93182, t25411, t136, t2457, t7423, t25299, t25431, t26555, t40270, t25305, t25410, t7419, t93240);
        let (t95813, t95822, t95825, t95836, t95859, t95862) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855(t26519, t93160, t25372, t95536, t7398, t822, t93170, t95746, t7064, t95575, t2067, t41117);
    (t95786, t95794, t95796, t95807, t95808, t95811, t95813, t95822, t95825, t95836, t95859, t95862)
}
