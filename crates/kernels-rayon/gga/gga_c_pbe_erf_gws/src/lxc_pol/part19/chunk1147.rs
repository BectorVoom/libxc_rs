//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1147/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1147(t2306: f64, t820: f64, t9385: f64, t3975: f64, t3972: f64, t13780: f64, t3060: f64, t3990: f64, t13859: f64, t1146: f64, t3955: f64, t13793: f64, t14657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14696 = t2306 * t820;
    let t14697 = t9385 * t14696;
    let t14698 = t3975 * t14697;
    let t14699 = t3972 * t14698;
    let t14705 = t3990 * t13780 * t3060;
    let t14706 = t13859 * t14705;
    let t14708 = t3955 * t1146;
    let t14714 = t14657 * t13793;
    (t14696, t14698, t14699, t14705, t14706, t14708, t14714)
}
