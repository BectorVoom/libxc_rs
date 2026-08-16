//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3240/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240(t18805: f64, t41066: f64, t10995: f64, t122: f64, t18796: f64, t2466: f64, t11044: f64, t18797: f64, t18317: f64, t2435: f64, t10770: f64, t14791: f64, t14917: f64, t18426: f64, t2724: f64, t2745: f64, t40337: f64, t40357: f64, t40361: f64, t4362: f64, t4364: f64, t50292: f64, t50296: f64, t50298: f64, t50303: f64, t50308: f64, t51049: f64, t6035: f64) -> (f64, f64, f64, f64, f64) {
    let t61430 = t41066 * t18805;
    let t61437 = t10995 * t18796 * t122 * t2466;
    let t61441 = t11044 * t18797;
    let t61448 = t2435 * t18317;
    let t61471 = -0.42874018118069736972e-2_f64 * t2745 * t10770 * t18426 * t14917 + 0.30011812682648815881e-2_f64 * t4362 * t4364 * t18426 * t2724 + 0.34299214494455789578e-2_f64 * t2745 * t14791 * t51049 * t6035 - 0.27104001498285508386e-3_f64 * t40337 + 0.13552000749142754193e-3_f64 * t40357 + 0.75585306015559980738e-1_f64 * t40361 + 0.10164000561857065645e-3_f64 * t50292 + 0.12004725073059526352e-1_f64 * t50296 - 0.2168320119862840671e-2_f64 * t50298 - 0.10164000561857065645e-3_f64 * t50303 + 0.14291339372689912324e-4_f64 * t50308;
    (t61430, t61437, t61441, t61448, t61471)
}
