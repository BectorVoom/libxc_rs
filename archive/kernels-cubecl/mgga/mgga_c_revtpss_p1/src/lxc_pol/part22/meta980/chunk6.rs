//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3308/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3308<F: Float>(t2782: F, t2797: F, t62695: F, t39649: F, t39652: F, t39662: F, t39673: F, t39678: F, t39683: F, t51390: F, t51403: F, t51408: F, t61648: F, t62682: F, t62684: F, t62693: F, t820: F, t879: F) -> F {
    let t62697 = t2782 * t2797 * t62695;
    let t62705 = -F::cast_from(0.19514881078765566038e-1_f64) * t62682 + t39649 - t39652 - F::cast_from(0.13009920719177044025e-2_f64) * t62684 + F::cast_from(0.520396828767081761e-2_f64) * t51390 - F::cast_from(0.2601984143835408805e-1_f64) * t39662 + F::cast_from(0.92526556154787137112e-2_f64) * t39673 - F::cast_from(0.11565819519348392139e-2_f64) * t39678 + F::cast_from(0.10975748638225852664e-1_f64) * t62693 + F::cast_from(0.10975748638225852664e-1_f64) * t62697 - F::cast_from(0.34146773541147097178e-1_f64) * t51403 + F::cast_from(0.23131639038696784278e-2_f64) * t39683 - F::cast_from(0.60712963356159538786e-1_f64) * t51408 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t879 * t61648;
    t62705
}
