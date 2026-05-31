//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2263/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2263<F: Float>(t114: F, t101468: F, t508: F, t651: F, t530: F, t7933: F, t2014: F, t25865: F, t1353: F, t22496: F, t28167: F, t8717: F, t25082: F, t73394: F) -> (F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t101469 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t101468);
    let t101472 = F::cast_from(2.0_f64) * t651 * t508 * t101469;
    let t101473 = t530 * t7933;
    let t101476 = F::cast_from(6.0_f64) * t2014 * t101473 * t25865;
    let t101479 = t22496 * t1353;
    let t101482 = F::cast_from(12.0_f64) * t28167 * t8717 * t101479;
    let t101485 = F::cast_from(6.0_f64) * t25082 * t8717 * t73394;
    (t101469, t101472, t101476, t101482, t101485)
}
