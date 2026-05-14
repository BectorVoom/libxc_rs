//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 564/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk564<F: Float>(t1104: F, t575: F, t1112: F, t687: F, t2970: F, t2976: F, t2984: F, t2988: F, t2991: F, t3002: F, t3009: F, t3014: F, t3019: F, t3023: F, t3025: F, t3029: F, t3032: F, t3037: F, t3040: F, t3047: F, t3049: F, t3051: F, t3054: F, t3058: F, t3062: F, t3066: F) -> (F, F, F, F) {
    let t3480 = t1104 * t575;
    let t3483 = t1112 * t687;
    let t3497 = 0.10821235962619981449e-3 * t2970 + 0.12163329537032409896e-2 * t2976 - 0.20241536458333333335e-4 * t2984 + 0.17376185052903442709e-3 * t2988 + 0.17376185052903442709e-3 * t2991 + 0.16882592796244404291e-6 * t3002 + 0.33765185592488808582e-6 * t3009 - 0.50680539737635041235e-4 * t3014 - 0.14492726735651760868e-5 * t3019 + 0.28985453471303521736e-5 * t3023 - 0.16908181191593721013e-4 * t3025;
    let t3509 = 0.14492726735651760868e-5 * t3029 + 0.12357942809624928455e-3 * t3032 - 0.25745714186718600948e-5 * t3037 + 0.2318836277704281739e-4 * t3040 + 0.21135226489492151266e-6 * t3047 - 0.4637672555408563478e-4 * t3049 + 0.4637672555408563478e-4 * t3051 + 0.38647271295071362317e-6 * t3054 - 0.68714848362636882201e-6 * t3058 - 0.16882592796244404291e-6 * t3062 - 0.16882592796244404291e-6 * t3066;
    (t3480, t3483, t3497, t3509)
}
