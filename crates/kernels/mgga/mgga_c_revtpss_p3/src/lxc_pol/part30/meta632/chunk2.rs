//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2200/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2200<F: Float>(t114: F, t101468: F, t508: F, t651: F, t530: F, t7933: F, t2014: F, t25865: F, t1353: F, t22496: F, t28167: F, t8717: F, t25082: F, t73394: F) -> (F, F, F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t101469 = piecewise3::<F>(t115, F::new(0.0), t101468);
    let t101472 = F::new(2.0) * t651 * t508 * t101469;
    let t101473 = t530 * t7933;
    let t101476 = F::new(6.0) * t2014 * t101473 * t25865;
    let t101479 = t22496 * t1353;
    let t101482 = F::new(12.0) * t28167 * t8717 * t101479;
    let t101485 = F::new(6.0) * t25082 * t8717 * t73394;
    (t101469, t101472, t101476, t101482, t101485)
}
