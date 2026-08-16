//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1100/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1100(t1971: f64, t236: f64, t6099: f64, t8517: f64, t10050: f64, t34857: f64, t1987: f64, t47854: f64, t1990: f64, t1979: f64, t1982: f64, t458: f64, t9774: f64) -> (f64, f64, f64, f64, f64) {
    let t47984 = t8517 * t1971 * t236 * t6099;
    let t47986 = t34857 * t10050;
    let t47988 = t47854 * t1987;
    let t47990 = t47854 * t1990;
    let t47994 = t9774 * t458 * t1979 * t1982;
    (t47984, t47986, t47988, t47990, t47994)
}
