//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1005/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1005(t2147: f64, t46976: f64, t1763: f64, t2084: f64, t27: f64, t7263: f64, t2191: f64, t9817: f64, t1986: f64, t6599: f64, t675: f64, t2310: f64, t9087: f64) -> (f64, f64, f64, f64, f64) {
    let t46977 = t46976 * t2147;
    let t46981 = t7263 * t27 * t2084 * t1763;
    let t46985 = t2191 * t9817;
    let t46989 = t675 * t1986 * t6599;
    let t46992 = t9087 * t2310;
    (t46977, t46981, t46985, t46989, t46992)
}
