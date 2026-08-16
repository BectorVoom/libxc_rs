//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1048/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1048(t11757: f64, t905: f64, t8996: f64, t9016: f64, t11744: f64, t858: f64, t3065: f64, t8978: f64, t3134: f64, t8881: f64, t8983: f64, t8897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11758 = t905 * t11757;
    let t11762 = t9016 * t8996 / 48.0_f64;
    let t11763 = t858 * t11744;
    let t11764 = t3065 * t11763;
    let t11766 = t8978 * t11764 / 96.0_f64;
    let t11768 = t8881 * t3134 / 48.0_f64;
    let t11770 = t8978 * t8983 / 48.0_f64;
    let t11772 = t9016 * t8897 / 24.0_f64;
    (t11758, t11762, t11764, t11766, t11768, t11770, t11772)
}
