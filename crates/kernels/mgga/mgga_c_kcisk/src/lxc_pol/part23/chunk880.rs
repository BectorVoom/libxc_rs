//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 880/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk880<F: Float>(t140: F, t446: F, t480: F, t1460: F, t306: F, t1474: F, t979: F, t4265: F, t4279: F, t4288: F, t4274: F, t1477: F, t430: F, t4284: F, t3904: F, t442: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14409 = 0.11791604938271604938e-1 * t140 * t446 * t480;
    let t14434 = t1460 * t306;
    let t14439 = t979 * t1474;
    let t14441 = t4265 * t4279;
    let t14444 = t4265 * t4288;
    let t14446 = t4265 * t4274;
    let t14469 = t140 * t430 * t1477;
    let t14489 = t4265 * t4284;
    let t14502 = t3904 * t442;
    (t14409, t14434, t14439, t14441, t14444, t14446, t14469, t14489, t14502)
}
