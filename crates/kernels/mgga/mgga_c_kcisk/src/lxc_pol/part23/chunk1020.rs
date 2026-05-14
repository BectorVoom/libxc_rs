//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1020/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1020<F: Float>(t1248: F, t19114: F, t4065: F, t13748: F, t20298: F, t20305: F, t20312: F, t20315: F, t20318: F, t20321: F, t20324: F, t20454: F, t20461: F, t13526: F, t13530: F, t13533: F, t13595: F, t13605: F, t13616: F, t13746: F, t20292: F, t20308: F, t20327: F, t20373: F, t20393: F, t20424: F, t20427: F, t20430: F, t20433: F, t20438: F, t20440: F, t20443: F, t20446: F, t20450: F) -> (F, F) {
    let t20465 = t1248 * t4065 * t19114;
    let t20467 = -0.40256666666666666667e0 * t20298 + t20454 - 0.20128333333333333333e0 * t20315 - 0.33547222222222222222e0 * t20305 - 0.80513333333333333333e0 * t20312 + 0.60385e0 * t20324 + 0.24154e1 * t20321 - 0.99342e0 * t20461 - 0.181155e1 * t20318 - t13748 + 0.33114e0 * t20465;
    let t20469 = -0.60385e0 * t20327 - 0.22076e0 * t13595 + 0.36793333333333333333e-1 * t13605 - 0.36793333333333333334e0 * t13616 - 0.18396666666666666667e0 * t20373 - 0.13418888888888888889e0 * t20292 + 0.16504875e0 * t20393 - 0.26837777777777777778e0 * t13526 + 0.67094444444444444447e-1 * t13530 - 0.20128333333333333334e0 * t13533 + t20424 - 0.258925e1 * t20427 + 0.33114e0 * t20430 + 0.132456e1 * t20433 + 0.12077e1 * t20308 - t13746 - t20438 + 0.73586666666666666666e-1 * t20440 - 0.5519e-1 * t20443 - 0.73586666666666666666e-1 * t20446 - 0.22076e0 * t20450 + t20467;
    (t20465, t20469)
}
