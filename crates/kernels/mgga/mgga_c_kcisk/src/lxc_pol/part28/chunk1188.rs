//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1188/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1188<F: Float>(t7268: F, t9670: F, t7261: F, t17010: F, t2781: F, t7278: F, t9656: F, t4830: F, t9931: F, t1763: F, t20: F, t2454: F, t1693: F, t1333: F, t9949: F, t9957: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34260 = t9670 * t7268;
    let t34261 = t7261 * t34260;
    let t34264 = t17010 * t2781;
    let t34267 = t7278 * t9656;
    let t34270 = t4830 * t9931;
    let t34274 = t1763 * t2454 * t20;
    let t34275 = t1693 * t34274;
    let t34278 = t1333 * t9949;
    let t34280 = t1333 * t9957;
    (t34260, t34261, t34264, t34267, t34270, t34274, t34275, t34278, t34280)
}
