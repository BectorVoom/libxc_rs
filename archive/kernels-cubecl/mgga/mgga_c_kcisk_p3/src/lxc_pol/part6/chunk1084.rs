//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1084/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1084<F: Float>(t294: F, t31804: F, t15452: F, t15463: F, t15473: F, t15763: F, t28152: F, t30148: F, t30149: F, t30151: F, t30173: F, t30176: F, t30181: F, t31803: F) -> F {
    let t31805 = t294 * t31804;
    let t31806 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t31805;
    let t31807 = -t30148 + t15452 + t30149 + t30151 + F::cast_from(3.0_f64) * t28152 - t15463 + t30173 - t30176 - t30181 - t31803 - t15473 + t15763 - t31806;
    t31807
}
