//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1354/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1354(t1484: f64, t22986: f64, t23270: f64, t98161: f64, t1880: f64, t7488: f64, t98133: f64, t105250: f64, t105254: f64, t1492: f64, t17052: f64, t259: f64, t28317: f64, t28406: f64, t4147: f64, t7517: f64, t82070: f64, t86911: f64, t86916: f64, t98213: f64) -> f64 {
    let t105258 = t22986 * t23270 * t98161 * t1484;
    let t105267 = t1880 * t98133 * t7488;
    let t105269 = 0.78134368175290755733e-1_f64 * t86911 + 0.49348022005446793095e-1_f64 * t86916 + t82070 + 6.0_f64 * t4147 * t28317 - 0.82246703342411321825e-2_f64 * t105250 - 0.49348022005446793095e-1_f64 * t105254 - 0.9869604401089358619e-1_f64 * t105258 - 0.49348022005446793095e-1_f64 * t98213 + 6.0_f64 * t17052 * t7517 + 3.0_f64 * t1492 * t28406 * t259 - 0.24674011002723396548e-1_f64 * t105267;
    t105269
}
