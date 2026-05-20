//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3240/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240<F: Float>(t18805: F, t41066: F, t10995: F, t122: F, t18796: F, t2466: F, t11044: F, t18797: F, t18317: F, t2435: F, t10770: F, t14791: F, t14917: F, t18426: F, t2724: F, t2745: F, t40337: F, t40357: F, t40361: F, t4362: F, t4364: F, t50292: F, t50296: F, t50298: F, t50303: F, t50308: F, t51049: F, t6035: F) -> (F, F, F, F, F) {
    let t61430 = t41066 * t18805;
    let t61437 = t10995 * t18796 * t122 * t2466;
    let t61441 = t11044 * t18797;
    let t61448 = t2435 * t18317;
    let t61471 = -F::cast_from(0.42874018118069736972e-2_f64) * t2745 * t10770 * t18426 * t14917 + F::cast_from(0.30011812682648815881e-2_f64) * t4362 * t4364 * t18426 * t2724 + F::cast_from(0.34299214494455789578e-2_f64) * t2745 * t14791 * t51049 * t6035 - F::cast_from(0.27104001498285508386e-3_f64) * t40337 + F::cast_from(0.13552000749142754193e-3_f64) * t40357 + F::cast_from(0.75585306015559980738e-1_f64) * t40361 + F::cast_from(0.10164000561857065645e-3_f64) * t50292 + F::cast_from(0.12004725073059526352e-1_f64) * t50296 - F::cast_from(0.2168320119862840671e-2_f64) * t50298 - F::cast_from(0.10164000561857065645e-3_f64) * t50303 + F::cast_from(0.14291339372689912324e-4_f64) * t50308;
    (t61430, t61437, t61441, t61448, t61471)
}
