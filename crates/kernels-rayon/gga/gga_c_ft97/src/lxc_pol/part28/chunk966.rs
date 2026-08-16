//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 966/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk966(t1882: f64, t32459: f64, t32500: f64, t32613: f64, t32555: f64, t32504: f64, t32471: f64, t32547: f64, t32479: f64, t32542: f64, t32581: f64, t376: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t137891 = t1882 * t32459;
    let t137900 = t1882 * t32500;
    let t137906 = t1882 * t32613;
    let t137908 = t1882 * t32555;
    let t137921 = t1882 * t32504;
    let t137923 = t1882 * t32471;
    let t137980 = t1882 * t32547;
    let t137987 = t1882 * t32479;
    let t137997 = t1882 * t32542;
    let t138000 = t89 * t376 * t32581;
    (t137891, t137900, t137906, t137908, t137921, t137923, t137980, t137987, t137997, t138000)
}
