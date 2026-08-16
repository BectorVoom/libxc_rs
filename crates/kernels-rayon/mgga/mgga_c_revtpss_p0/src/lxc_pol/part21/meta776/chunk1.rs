//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2767/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767(t50868: f64, t14325: f64, t14622: f64, t40156: f64, t14440: f64, t2398: f64, t40172: f64, t40178: f64, t14369: f64, t2258: f64, t4401: f64, t14370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50869 = 72.0_f64 * t50868;
    let t50871 = 36.0_f64 * t14325 * t14622;
    let t50872 = 0.51947577317044391277e2_f64 * t40156;
    let t50873 = t2398 * t14440;
    let t50874 = 12.0_f64 * t50873;
    let t50875 = 0.30762056574649219973e4_f64 * t40172;
    let t50876 = 36.0_f64 * t40178;
    let t50878 = t4401 * t14369 * t2258;
    let t50879 = 36.0_f64 * t50878;
    let t50880 = t14325 * t14370;
    (t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880)
}
