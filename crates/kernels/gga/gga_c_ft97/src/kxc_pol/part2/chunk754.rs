//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 754/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk754<F: Float>(t13352: F, t3910: F, t1091: F, t2373: F, t9896: F, t1131: F, t2360: F, t2349: F, t2493: F, t3930: F, t9707: F, t2: F, t3821: F, t2372: F, t713: F, t1934: F, t3712: F) -> (F, F, F, F, F, F, F, F) {
    let t13353 = t3910 * t13352;
    let t13356 = t1091 * t2373;
    let t13357 = t9896 * t13356;
    let t13360 = t1131 * t2360;
    let t13361 = t13360 * t2349;
    let t13362 = t2493 * t13361;
    let t13370 = t9707 * t3930 * t2373;
    let t13373 = t2 * t3821;
    let t13375 = t2372 * t13373 * t713;
    let t13378 = t3712 * t1934;
    (t13353, t13356, t13357, t13361, t13362, t13370, t13375, t13378)
}
