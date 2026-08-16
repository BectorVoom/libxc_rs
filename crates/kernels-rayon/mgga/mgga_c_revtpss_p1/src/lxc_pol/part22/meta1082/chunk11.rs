//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3912/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3912(t22307: f64, t545: f64, t689: f64, t869: f64, t14239: f64, t14242: f64, t10023: f64, t22314: f64, t2470: f64, t22009: f64, t4004: f64, t4057: f64, t47348: f64, t49248: f64, t49252: f64, t49256: f64, t49260: f64, t49263: f64, t49273: f64, t5745: f64, t5755: f64, t9840: f64) -> f64 {
    let t75174 = t689 * t869 * t545 * t22307;
    let t75176 = t14239 * t14242;
    let t75179 = t10023 * t22314 * t2470;
    let t75182 = 0.39512695097613069591e1_f64 * t5745 * t22009 * t4004 - 0.11708928647259339623e0_f64 * t49248 - 0.10975748638225852664e-1_f64 * t49252 - 0.1040793657534163522e0_f64 * t49256 + 0.78059524315062264152e-1_f64 * t49260 - 0.65854491829355115987e0_f64 * t5755 * t22009 * t4057 + 0.13170898365871023197e1_f64 * t5745 * t22009 * t9840 - 0.43902994552903410656e-1_f64 * t49263 - 0.52039682876708176102e-1_f64 * t49273 - 0.10975748638225852664e-1_f64 * t75174 + 0.26019841438354088049e-1_f64 * t75176 - 0.26019841438354088049e-1_f64 * t75179 + 0.39274398764404314548e-3_f64 * t47348;
    t75182
}
