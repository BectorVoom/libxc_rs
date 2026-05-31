//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1207/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1207<F: Float>(t30950: F, t30973: F, t3: F, t1918: F, t2170: F, t30180: F, t30182: F, t30184: F, t30187: F, t30190: F, t30193: F, t30196: F, t573: F, t6945: F, t6948: F, t8245: F, param_d: F) -> (F, F, F, F) {
    let t30974 = t30950 + t30973;
    let t30975 = t3 * t30974;
    let t30985 = param_d * t30974;
    let t30993 = F::cast_from(6.0_f64) * t1918 * t8245 + F::cast_from(6.0_f64) * t2170 * t6945 + F::cast_from(3.0_f64) * t2170 * t6948 + t30985 * t573 + t30180 + t30182 + t30184 + t30187 + t30190 + t30193 + t30196;
    (t30974, t30975, t30985, t30993)
}
