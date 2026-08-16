//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1276/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1276(t13776: f64, t3212: f64, t50956: f64, t875: f64, t51651: f64, t13812: f64, t2503: f64, t13817: f64, t14418: f64, t859: f64, t892: f64, t1177: f64, t1178: f64, t371: f64, t9689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53748 = t13776 * t50956 * t3212 * t875;
    let t53750 = 35.0_f64 / 108.0_f64 * t51651;
    let t53751 = t13812 * t2503;
    let t53758 = t13817 * t2503;
    let t53761 = t859 * t892 * t14418;
    let t53768 = t1177 * t371 * t1178 * t9689;
    (t53748, t53750, t53751, t53758, t53761, t53768)
}
