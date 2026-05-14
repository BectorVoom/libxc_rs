//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1026/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1026<F: Float>(t20604: F, t395: F, t1300: F, t6148: F, t1305: F, t6149: F, t13482: F, t13493: F, t20255: F, t20258: F, t20261: F, t3935: F, t3939: F, t405: F, t6176: F, t6180: F, t6184: F, sigma0: F) -> (F,) {
    let t20605 = t20604 * sigma0;
    let t20606 = t20605 * t395;
    let t20609 = t6148 * t1300;
    let t20613 = 0.35981577432354634426e-1 * t6149 * t1305;
    let t20614 = 0.95950873152945691802e-1 * t13482 * t6180 + 0.1919017463058913836e0 * t13482 * t6184 - 0.1279344975372609224e0 * t13482 * t6176 - 0.35981577432354634426e-1 * t13493 * t6180 - 0.71963154864709268852e-1 * t13493 * t6184 + 0.47975436576472845902e-1 * t13493 * t6176 - 0.35981577432354634426e-1 * t20255 * t3939 + 0.10794473229706390328e0 * t3935 * t20258 - 0.1439263097294185377e0 * t3935 * t20261 + 0.5397236614853195164e-1 * t20606 * t405 - 0.28785261945883707542e0 * t20609 * t405 + t20613;
    (t20614,)
}
