//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1051/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1051<F: Float>(t1492: F, t6376: F, t1487: F, t14344: F, t6328: F, t2259: F, t4175: F, t6373: F, t19848: F, t492: F, t4237: F, t4200: F, t6388: F, t19886: F, t4231: F, t6368: F) -> (F, F, F, F, F, F, F, F) {
    let t21057 = t1492 * t6376;
    let t21058 = t1487 * t21057;
    let t21060 = t14344 * t6328;
    let t21062 = t2259 * t4175;
    let t21064 = t14344 * t6373;
    let t21066 = t19848 * t492;
    let t21067 = t21066 * t4237;
    let t21069 = t6388 * t4200;
    let t21071 = t4231 * t19886;
    let t21072 = t6368 * t21071;
    (t21058, t21060, t21062, t21064, t21067, t21069, t21071, t21072)
}
