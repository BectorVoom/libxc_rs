//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1325/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1325(t3123: f64, t8897: f64, t8996: f64, t11542: f64, t51470: f64, t11554: f64, t14015: f64, t11764: f64, t54119: f64, t11560: f64, t14007: f64, t11526: f64, t51421: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56954 = t3123 * t8897;
    let t56956 = t3123 * t8996;
    let t56958 = t51470 * t11542;
    let t56960 = t14015 * t11554;
    let t56962 = t54119 * t11764;
    let t56964 = t14007 * t11560;
    let t56966 = t51421 * t11526;
    (t56954, t56956, t56958, t56960, t56962, t56964, t56966)
}
