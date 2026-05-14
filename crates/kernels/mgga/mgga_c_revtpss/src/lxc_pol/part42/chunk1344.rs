//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1344/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1344<F: Float>(t3: F, t31700: F, t2198: F, t5883: F, t1518: F, t31505: F, t5920: F, t8342: F, t117: F, t31653: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t6941: F, t6945: F, t6948: F, t8421: F, t8427: F, t8430: F) -> (F, F, F, F, F, F, F) {
    let t31701 = t3 * t31700;
    let t31711 = param_d * t31700;
    let t31725 = t5883 * t2198;
    let t31728 = t31505 * t1518;
    let t31731 = t8342 * t5920;
    let t31734 = t117 * t31653;
    let t31737 = 12.0 * t1916 * t8427 + 6.0 * t1916 * t8430 + 6.0 * t1918 * t8421 + 6.0 * t2207 * t6945 + 3.0 * t2207 * t6948 + 3.0 * t2209 * t6941 + t31711 * t573 + 6.0 * t31725 * t572 + 12.0 * t31728 * t572 + 6.0 * t31731 * t572 + 3.0 * t31734 * t572;
    (t31701, t31711, t31725, t31728, t31731, t31734, t31737)
}
