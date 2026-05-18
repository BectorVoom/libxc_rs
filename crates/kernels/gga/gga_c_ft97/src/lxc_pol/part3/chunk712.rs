//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 712/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk712<F: Float>(t1095: F, t2378: F, t25: F, t695: F, t677: F, t2393: F, t1113: F, t2426: F, t51: F, t6032: F, t3771: F, t236: F, t3750: F) -> (F, F, F, F, F, F, F) {
    let t13469 = t2378 * t1095;
    let t13473 = t695 * t25;
    let t13474 = t677 * t13473;
    let t13475 = t2393 * t1095;
    let t13491 = t2426 * t1113;
    let t13519 = t6032 * t51;
    let t13520 = t3771 * t13519;
    let t13526 = t236 * t3750;
    (t13469, t13473, t13474, t13475, t13491, t13520, t13526)
}
