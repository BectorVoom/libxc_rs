//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1210/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1210<F: Float>(t32435: F, t7290: F, t1841: F, t7289: F, t10755: F, t5288: F, t10683: F, t7129: F, t1897: F, t2717: F, t8942: F, t10643: F) -> (F, F, F, F, F, F) {
    let t32436 = t7290 * t32435;
    let t32439 = F::cast_from(0.34180116578409885704e-2_f64) * t1841 * t7289 * t32436;
    let t32441 = F::cast_from(0.15381052460284448567e-1_f64) * t5288 * t10755;
    let t32443 = F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t10683;
    let t32446 = F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t2717 * t8942;
    let t32448 = F::cast_from(0.10766736722199113997e0_f64) * t7129 * t10643;
    (t32436, t32439, t32441, t32443, t32446, t32448)
}
