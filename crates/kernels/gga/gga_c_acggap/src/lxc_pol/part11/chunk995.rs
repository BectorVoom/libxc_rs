//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 995/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk995<F: Float>(t13364: F, t31195: F, t35420: F, t31231: F, t7637: F, t8545: F, t31212: F, t31222: F, t31224: F, t31227: F, t31230: F, t31237: F, t31239: F, t31241: F, t31245: F, t31247: F, t35404: F, t35408: F, t35411: F, t35415: F, t35418: F) -> (F,) {
    let t35422 = t31195 * t13364 * t35420;
    let t35424 = 0.34299214494455789578e-2 * t31231;
    let t35425 = t7637 * t8545;
    let t35432 = t35404 + 0.28303283060643736861e-1 * t31212 - t35408 - t35411 - 0.42874018118069736972e-3 * t31222 - t35415 / 32.0 - 0.4501771902397322382e-1 * t31224 + t31227 + 0.33020496904084359671e-1 * t35418 + 0.10718504529517434243e-2 * t35422 + t31230 + t35424 + 0.47637797908966374413e-2 * t35425 - 0.31448092289604152068e-3 * t31237 - 0.31448092289604152068e-3 * t31239 - 0.83861579438944405513e-3 * t31241 + 0.31448092289604152068e-3 * t31245 + 0.32155513588552302729e-2 * t31247;
    (t35432,)
}
