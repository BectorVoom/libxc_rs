//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1356/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1356(t14688: f64, t2503: f64, t13796: f64, t13859: f64, t2171: f64, t56296: f64, t3959: f64, t9928: f64, t14121: f64, t9948: f64, t15282: f64, t51666: f64) -> (f64, f64, f64, f64, f64) {
    let t57326 = t14688 * t2503;
    let t57330 = t13859 * t13796 * t56296 * t2171;
    let t57332 = t3959 * t9928;
    let t57334 = t14121 * t9948;
    let t57338 = t51666 * t15282;
    (t57326, t57330, t57332, t57334, t57338)
}
