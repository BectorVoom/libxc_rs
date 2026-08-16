//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 907/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk907(t3873: f64, t6964: f64, t1319: f64, t11455: f64, t11608: f64, t11609: f64, t21268: f64, t21270: f64, t21273: f64, t21275: f64, t21278: f64, t21281: f64, t21283: f64) -> (f64, f64) {
    let t21285 = t3873 * t6964;
    let t21286 = t21285 * t1319;
    let t21288 = -0.91285185185185185187e-1_f64 * t11455 - t11608 - t11609 + 0.1898925e1_f64 * t21268 + 0.3071625e0_f64 * t21270 + 0.142419375e1_f64 * t21273 - 0.1898925e1_f64 * t21275 - 0.9494625e0_f64 * t21278 - 0.76790625e-1_f64 * t21281 + 0.3071625e0_f64 * t21283 + 0.15358125e0_f64 * t21286;
    (t21286, t21288)
}
