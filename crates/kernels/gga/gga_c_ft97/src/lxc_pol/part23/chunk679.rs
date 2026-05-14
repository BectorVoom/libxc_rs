//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 679/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk679<F: Float>(t13464: F, t13468: F, t13469: F, t13474: F, t13475: F, t1417: F, t1701: F, t17933: F, t17937: F, t17941: F, t17945: F, t17946: F, t17950: F, t17958: F, t17960: F, t17964: F, t17966: F, t17971: F, t17976: F, t17980: F, t17987: F, t17988: F, t2035: F, t2387: F, t3786: F, t6757: F) -> (F,) {
    let t17992 = -0.23254900946437792e-2 * t2387 * t17933 + 0.23254900946437792e-1 * t2387 * t17937 + 0.23254900946437792e-1 * t2387 * t17941 + 0.77462893625097599762e-3 * t17945 * t13469 * t17946 - 0.38731446812548799881e-3 * t13468 * t13469 * t17950 - 0.46509801892875584e-2 * t13474 * t13475 * t17950 - 0.46509801892875584e-1 * t17958 * t6757 * t17960 + 0.46509801892875584e-1 * t17964 * t6757 * t17966 + 0.93019603785751168e-2 * t17971 * t13475 * t17946 + 0.37540077436335915588e-1 * t1417 * t1701 * t17976 - 0.11854761295685025975e-1 * t1417 * t1701 * t17980 + 0.46509801892875584e-1 * t13464 * t3786 - 0.14053536537767171586e-3 * t17987 * t2035 * t17988;
    (t17992,)
}
