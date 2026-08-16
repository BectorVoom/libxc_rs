//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1337/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1337<F: Float>(t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F) -> (F, F, F, F, F, F) {
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = F::cast_from(0.3086419753086419753e-3_f64) * t339 * t10337;
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    (t10333, t10335, t10337, t10339, t10372, t10375)
}
