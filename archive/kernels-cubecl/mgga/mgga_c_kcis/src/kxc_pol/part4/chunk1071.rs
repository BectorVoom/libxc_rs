//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1071/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1071<F: Float>(t13516: F, t345: F, t1064: F, t13511: F, t945: F, t1079: F, t1056: F, t4898: F, t738: F, t4901: F, t13475: F, t104: F, t111: F, t120: F, t13485: F, t13488: F, t13492: F, t13493: F, t13496: F, t13499: F, t13502: F, t13504: F, t13507: F, t13512: F, t4858: F, t4865: F, t4881: F) -> F {
    let t13517 = t345 * t13516;
    let t13520 = t1064 * t13511;
    let t13523 = t945 * t13516;
    let t13526 = t1079 * t13511;
    let t13529 = t1056 * t13516;
    let t13532 = t738 * t4898;
    let t13535 = F::cast_from(0.17611111111111111111e-2_f64) * t738 * t4901;
    let t13536 = t1064 * t13475;
    let t13539 = -F::cast_from(0.672175e-5_f64) * t120 * t13485 + F::cast_from(0.22405833333333333333e-5_f64) * t120 * t13488 - t13492 - F::cast_from(0.31226666666666666666e-2_f64) * t13493 + F::cast_from(0.4755e-2_f64) * t111 * t13496 - F::cast_from(0.11955719325063177623e0_f64) * t13499 + F::cast_from(0.72513544709148296264e-3_f64) * t13502 - F::cast_from(0.21078e-1_f64) * t104 * t13504 + F::cast_from(0.30247875e-4_f64) * t120 * t13507 + F::cast_from(0.28104e-1_f64) * t4858 * t13512 + F::cast_from(0.4684e-2_f64) * t4858 * t13517 - F::cast_from(0.634e-2_f64) * t4865 * t13520 - F::cast_from(0.21133333333333333334e-2_f64) * t4865 * t13523 - F::cast_from(0.403305e-4_f64) * t4881 * t13526 - F::cast_from(0.26887e-4_f64) * t4881 * t13529 + F::cast_from(0.52833333333333333333e-2_f64) * t13532 + t13535 - F::cast_from(0.1585e-2_f64) * t111 * t13536;
    t13539
}
