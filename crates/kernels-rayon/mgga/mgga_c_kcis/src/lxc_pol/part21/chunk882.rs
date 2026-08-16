//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 882/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk882(t13516: f64, t345: f64, t1064: f64, t13511: f64, t945: f64, t1079: f64, t1056: f64, t4898: f64, t738: f64, t4901: f64, t13475: f64, t104: f64, t111: f64, t120: f64, t13485: f64, t13488: f64, t13492: f64, t13493: f64, t13496: f64, t13499: f64, t13502: f64, t13504: f64, t13507: f64, t13512: f64, t4858: f64, t4865: f64, t4881: f64) -> f64 {
    let t13517 = t345 * t13516;
    let t13520 = t1064 * t13511;
    let t13523 = t945 * t13516;
    let t13526 = t1079 * t13511;
    let t13529 = t1056 * t13516;
    let t13532 = t738 * t4898;
    let t13535 = 0.17611111111111111111e-2_f64 * t738 * t4901;
    let t13536 = t1064 * t13475;
    let t13539 = -0.672175e-5_f64 * t120 * t13485 + 0.22405833333333333333e-5_f64 * t120 * t13488 - t13492 - 0.31226666666666666666e-2_f64 * t13493 + 0.4755e-2_f64 * t111 * t13496 - 0.11955719325063177623e0_f64 * t13499 + 0.72513544709148296264e-3_f64 * t13502 - 0.21078e-1_f64 * t104 * t13504 + 0.30247875e-4_f64 * t120 * t13507 + 0.28104e-1_f64 * t4858 * t13512 + 0.4684e-2_f64 * t4858 * t13517 - 0.634e-2_f64 * t4865 * t13520 - 0.21133333333333333334e-2_f64 * t4865 * t13523 - 0.403305e-4_f64 * t4881 * t13526 - 0.26887e-4_f64 * t4881 * t13529 + 0.52833333333333333333e-2_f64 * t13532 + t13535 - 0.1585e-2_f64 * t111 * t13536;
    t13539
}
