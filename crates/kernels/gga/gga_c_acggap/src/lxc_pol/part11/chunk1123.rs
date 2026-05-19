//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1123/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1123<F: Float>(t2299: F, t7780: F, t3196: F, t33953: F, t13364: F, t31195: F, t31231: F, t7637: F, t8545: F, t31212: F, t31222: F, t31224: F, t31227: F, t31230: F, t31237: F, t31239: F, t31241: F, t31245: F, t31247: F, t35404: F, t35408: F, t35411: F, t35415: F) -> (F, F) {
    let t35418 = t7780 * t2299;
    let t35420 = t33953 * t3196;
    let t35422 = t31195 * t13364 * t35420;
    let t35424 = F::cast_from(0.34299214494455789578e-2_f64) * t31231;
    let t35425 = t7637 * t8545;
    let t35432 = t35404 + F::cast_from(0.28303283060643736861e-1_f64) * t31212 - t35408 - t35411 - F::cast_from(0.42874018118069736972e-3_f64) * t31222 - t35415 / F::new(32.0) - F::cast_from(0.4501771902397322382e-1_f64) * t31224 + t31227 + F::cast_from(0.33020496904084359671e-1_f64) * t35418 + F::cast_from(0.10718504529517434243e-2_f64) * t35422 + t31230 + t35424 + F::cast_from(0.47637797908966374413e-2_f64) * t35425 - F::cast_from(0.31448092289604152068e-3_f64) * t31237 - F::cast_from(0.31448092289604152068e-3_f64) * t31239 - F::cast_from(0.83861579438944405513e-3_f64) * t31241 + F::cast_from(0.31448092289604152068e-3_f64) * t31245 + F::cast_from(0.32155513588552302729e-2_f64) * t31247;
    (t35420, t35432)
}
