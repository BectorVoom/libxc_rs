//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 905/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk905<F: Float>(t17975: F, t709: F, t3780: F, t3817: F, t39: F, t695: F, t224: F, t5266: F, t13464: F, t13468: F, t13469: F, t13474: F, t13475: F, t1417: F, t1701: F, t17933: F, t17937: F, t17941: F, t17945: F, t17946: F, t17950: F, t17958: F, t17960: F, t17964: F, t17966: F, t17971: F, t2035: F, t2387: F, t3786: F, t6757: F) -> F {
    let t17976 = t17975 * t709;
    let t17980 = t3780 * t3817;
    let t17986 = t695 * t39;
    let t17987 = t224 * t17986;
    let t17988 = t5266 * t709;
    let t17992 = -F::cast_from(0.23254900946437792e-2_f64) * t2387 * t17933 + F::cast_from(0.23254900946437792e-1_f64) * t2387 * t17937 + F::cast_from(0.23254900946437792e-1_f64) * t2387 * t17941 + F::cast_from(0.77462893625097599762e-3_f64) * t17945 * t13469 * t17946 - F::cast_from(0.38731446812548799881e-3_f64) * t13468 * t13469 * t17950 - F::cast_from(0.46509801892875584e-2_f64) * t13474 * t13475 * t17950 - F::cast_from(0.46509801892875584e-1_f64) * t17958 * t6757 * t17960 + F::cast_from(0.46509801892875584e-1_f64) * t17964 * t6757 * t17966 + F::cast_from(0.93019603785751168e-2_f64) * t17971 * t13475 * t17946 + F::cast_from(0.37540077436335915588e-1_f64) * t1417 * t1701 * t17976 - F::cast_from(0.11854761295685025975e-1_f64) * t1417 * t1701 * t17980 + F::cast_from(0.46509801892875584e-1_f64) * t13464 * t3786 - F::cast_from(0.14053536537767171586e-3_f64) * t17987 * t2035 * t17988;
    t17992
}
