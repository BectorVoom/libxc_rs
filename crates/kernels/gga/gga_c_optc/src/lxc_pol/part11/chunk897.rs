//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 897/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk897<F: Float>(t16917: F, t4039: F, t2722: F, t4044: F, t3608: F, t10595: F, t13794: F, t13803: F, t13912: F, t1435: F, t16676: F, t16902: F, t16912: F, t277: F, t4038: F, t4054: F, t5065: F, t7332: F, t8393: F, t95: F, t999: F) -> (F, F, F, F, F) {
    let t16918 = t4039 * t16917;
    let t16919 = t2722 * t16918;
    let t16921 = t4044 * t16917;
    let t16922 = t3608 * t16921;
    let t16925 = F::new(14.0) / F::new(27.0) * t999 * t16902 + t13912 * t1435 / F::new(2.0) + t4054 * t5065 / F::new(2.0) + t13794 / F::new(2.0) + F::new(2.0) / F::new(9.0) * t13803 - t10595 / F::new(9.0) + t16676 + F::new(0.51689762869806860992e-2) * t95 * t277 * t16912 * t8393 + t7332 - t4038 * t16919 + F::new(2.0) / F::new(3.0) * t4038 * t16922;
    (t16918, t16919, t16921, t16922, t16925)
}
