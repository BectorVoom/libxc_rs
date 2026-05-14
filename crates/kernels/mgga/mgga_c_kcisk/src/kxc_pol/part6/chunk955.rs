//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 955/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk955<F: Float>(t240: F, t31197: F, t31199: F, t31201: F, t31203: F, t31206: F, t31209: F, t31406: F, t31798: F, t297: F, t294: F, t2351: F, t7728: F, t15452: F, t15463: F, t15473: F, t15763: F, t28152: F, t30148: F, t30149: F, t30151: F, t30173: F, t30176: F, t30181: F) -> (F,) {
    let t31800 = t240 * t31798 + t31197 - t31199 + t31201 - t31203 - t31206 + t31209 - t31406;
    let t31801 = t297 * t31800;
    let t31802 = t294 * t31801;
    let t31803 = t31802 / 16.0;
    let t31804 = t7728 * t2351;
    let t31805 = t294 * t31804;
    let t31806 = 3.0 / 16.0 * t31805;
    let t31807 = -t30148 + t15452 + t30149 + t30151 + 3.0 * t28152 - t15463 + t30173 - t30176 - t30181 - t31803 - t15473 + t15763 - t31806;
    (t31807,)
}
