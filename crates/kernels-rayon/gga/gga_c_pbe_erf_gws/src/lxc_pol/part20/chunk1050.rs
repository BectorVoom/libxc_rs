//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1050/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1050(t11785: f64, t337: f64, t2121: f64, t6645: f64, t11746: f64, t11750: f64, t11754: f64, t11758: f64, t11762: f64, t11766: f64, t11768: f64, t11770: f64, t11772: f64, t11775: f64, t11780: f64, t11784: f64, t6637: f64, t902: f64) -> (f64, f64) {
    let t11786 = t337 * t11785;
    let t11787 = t2121 * t11786;
    let t11789 = t6645 * t11787 / 48.0_f64;
    let t11790 = t6637 * t11746 / 768.0_f64 - t6637 * t11750 / 384.0_f64 + t902 * t11754 / 1536.0_f64 + t902 * t11758 / 1536.0_f64 - t11762 + t11766 - t11768 + t11770 + t11772 - t11775 - t11780 - t11784 + t11789;
    (t11789, t11790)
}
