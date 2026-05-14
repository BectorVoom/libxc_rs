//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 571/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk571<F: Float>(t1117: F, t883: F, t1125: F, t972: F, t3274: F, t3276: F, t3279: F, t3282: F, t3286: F, t3290: F, t3298: F, t3301: F, t3305: F, t3308: F, t3310: F, t3314: F, t3316: F, t3318: F, t3323: F, t3331: F, t3334: F, t3338: F, t3341: F, t3346: F, t3349: F, t3351: F) -> (F, F, F, F) {
    let t3565 = t1117 * t883;
    let t3568 = t1125 * t972;
    let t3582 = -0.3373480902777777778e-5 * t3274 - 0.16908181191593721013e-4 * t3276 + 0.14492726735651760868e-5 * t3279 + 0.12357942809624928455e-3 * t3282 + 0.28985453471303521736e-5 * t3286 - 0.28985453471303521736e-5 * t3290 + 0.14758978949652777779e-5 * t3298 - 0.50680539737635041235e-4 * t3301 - 0.14492726735651760868e-5 * t3305 + 0.27801896084645508334e-2 * t3308 + 0.27801896084645508334e-2 * t3310;
    let t3594 = 0.10120442708333333334e-4 * t3314 - 0.5060221354166666667e-4 * t3316 - 0.64871090864172852779e-2 * t3318 - 0.50027140879067581468e-8 * t3323 - 0.24619655944423022376e-7 * t3331 + 0.21135226489492151266e-6 * t3334 + 0.17376185052903442709e-3 * t3338 + 0.17376185052903442709e-3 * t3341 - 0.25745714186718600948e-5 * t3346 + 0.2318836277704281739e-4 * t3349 - 0.4637672555408563478e-4 * t3351;
    (t3565, t3568, t3582, t3594)
}
