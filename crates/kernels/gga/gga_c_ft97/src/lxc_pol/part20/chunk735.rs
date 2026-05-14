//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 735/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk735<F: Float>(t10915: F, t294: F, t3691: F, t2917: F, t3700: F, t18: F, t2639: F, t342: F, t5202: F, t630: F, t231: F, t4129: F, t10207: F, t10209: F, t10212: F, t13616: F, t1526: F, t15567: F, t18959: F, t2320: F, t343: F, t4027: F, t4037: F, t4052: F, t4135: F) -> (F, F, F, F, F, F, F) {
    let t18961 = t10915 * t294;
    let t18962 = t18961 * t3691;
    let t18968 = t2917 * t294;
    let t18969 = t18968 * t3700;
    let t18972 = t2639 * t18;
    let t18977 = t342 * t630 * t5202;
    let t18982 = t231 * t4129;
    let t18986 = t4027 + t4135 + t10207 - t10209 / 36.0 - t10212 / 12.0 - t18959 / 36.0 - t15567 * t18962 / 9.0 - t1526 * t2320 * t4037 / 12.0 + t15567 * t18969 / 6.0 + t1526 * t13616 * t18972 / 6.0 - t18977 / 12.0 - t1526 * t2320 * t4052 / 12.0 - t342 * t343 * t18982 / 4.0;
    (t18961, t18962, t18968, t18969, t18972, t18982, t18986)
}
