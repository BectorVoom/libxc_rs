//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 740/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk740<F: Float>(t17612: F, t17673: F, t184: F, t21: F, t1614: F, t51: F, t4417: F, t7712: F, t7720: F, t1528: F, t4431: F, t4495: F, t72: F, t1526: F, t1527: F, t15562: F, t15584: F, t3088: F, t342: F, t343: F, t4415: F, t4422: F, t4501: F, t7704: F) -> (F, F, F, F, F, F, F) {
    let t17674 = t17612 + t17673;
    let t17675 = t17674 * t184;
    let t17676 = t17675 * t21;
    let t17839 = t51 * t1614;
    let t19950 = t7712 * t4417;
    let t19957 = t7720 * t4417;
    let t19961 = t1528 * t4431;
    let t19965 = t72 * t4495;
    let t19969 = t4415 + t4501 + t7704 - t15562 / 18.0 - t15584 / 6.0 - t1526 * t3088 * t19950 / 9.0 - t1526 * t1527 * t4422 / 6.0 + t1526 * t1527 * t19957 / 6.0 - t1526 * t1527 * t19961 / 12.0 - t342 * t343 * t19965 / 4.0;
    (t17676, t17839, t19950, t19957, t19961, t19965, t19969)
}
