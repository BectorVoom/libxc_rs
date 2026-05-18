//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1327/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1327<F: Float>(t114: F, t101451: F, t105870: F, t105878: F, t114394: F, t114396: F, t114398: F, t94974: F, t1312: F, t2014: F, t2034: F, t86825: F, t1843: F, t30004: F, t651: F) -> (F, F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t114401 = piecewise3::<f64>(t115, F::new(0.0), -t94974 - F::new(11.0) / F::new(3.0) * t101451 - F::new(2.0) * t105870 + t105878 - F::new(3.0) / F::new(4.0) * t114394 + F::new(3.0) / F::new(4.0) * t114396 - t114398 / F::new(8.0));
    let t114403 = F::new(2.0) * t1312 * t114401;
    let t114407 = t2014 * t2034 * t86825;
    let t114410 = F::new(6.0) * t651 * t1843 * t30004;
    (t114401, t114403, t114407, t114410)
}
