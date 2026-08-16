//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1196/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1196<F: Float>(t32171: F, t10760: F, t7137: F, t10718: F, t10770: F, t2958: F, t7112: F, t2508: F, t2580: F, t2530: F, t8469: F, t24339: F, t935: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32172 = F::cast_from(0.64087718584518535698e-3_f64) * t32171;
    let t32185 = F::cast_from(0.6152420984113779427e-1_f64) * t7137 * t10760;
    let t32201 = F::cast_from(0.41016139894091862846e-1_f64) * t7137 * t10718;
    let t32207 = F::cast_from(0.20508069947045931424e-1_f64) * t7137 * t10770;
    let t32210 = t2958 * t7112;
    let t32213 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t32210;
    let t32219 = t8469 * t2530;
    let t32222 = F::cast_from(0.30762104920568897134e-1_f64) * t2508 * t2580 * t32219;
    let t32223 = t24339 * t935;
    (t32172, t32185, t32201, t32207, t32210, t32213, t32219, t32222, t32223)
}
