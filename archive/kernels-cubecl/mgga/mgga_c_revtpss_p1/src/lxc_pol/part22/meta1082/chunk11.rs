//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3912/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3912<F: Float>(t22307: F, t545: F, t689: F, t869: F, t14239: F, t14242: F, t10023: F, t22314: F, t2470: F, t22009: F, t4004: F, t4057: F, t47348: F, t49248: F, t49252: F, t49256: F, t49260: F, t49263: F, t49273: F, t5745: F, t5755: F, t9840: F) -> F {
    let t75174 = t689 * t869 * t545 * t22307;
    let t75176 = t14239 * t14242;
    let t75179 = t10023 * t22314 * t2470;
    let t75182 = F::cast_from(0.39512695097613069591e1_f64) * t5745 * t22009 * t4004 - F::cast_from(0.11708928647259339623e0_f64) * t49248 - F::cast_from(0.10975748638225852664e-1_f64) * t49252 - F::cast_from(0.1040793657534163522e0_f64) * t49256 + F::cast_from(0.78059524315062264152e-1_f64) * t49260 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t22009 * t4057 + F::cast_from(0.13170898365871023197e1_f64) * t5745 * t22009 * t9840 - F::cast_from(0.43902994552903410656e-1_f64) * t49263 - F::cast_from(0.52039682876708176102e-1_f64) * t49273 - F::cast_from(0.10975748638225852664e-1_f64) * t75174 + F::cast_from(0.26019841438354088049e-1_f64) * t75176 - F::cast_from(0.26019841438354088049e-1_f64) * t75179 + F::cast_from(0.39274398764404314548e-3_f64) * t47348;
    t75182
}
