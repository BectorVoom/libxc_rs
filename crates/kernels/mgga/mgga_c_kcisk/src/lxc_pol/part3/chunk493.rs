//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 493/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk493<F: Float>(t385: F, t3777: F, t4143: F, t1284: F, t3502: F, t1280: F, t1287: F, t340: F, t379: F, t382: F, t4134: F, t395: F, t1309: F, t1315: F, t1324: F, t3935: F, t3939: F, t3944: F, t3948: F, t3955: F, t3963: F, t3966: F, t3970: F, t3975: F, t3983: F, t3985: F, t3990: F, t3993: F, t3996: F, t4001: F, t4004: F, t405: F, sigma0: F) -> (F, F, F, F, F) {
    let t386 = t385 < -0.66725e-1;
    let t4144 = t4143 * t3777;
    let t4148 = t1284 * t3502;
    let t4153 = piecewise3(t386, 0.0, 10.0 / 9.0 * t340 * t4134 * t382 - 20.0 / 27.0 * t340 * t1280 * t1287 + 40.0 / 81.0 * t340 * t379 * t4144 - 10.0 / 27.0 * t340 * t379 * t4148);
    let t4154 = t4153 * sigma0;
    let t4155 = t4154 * t395;
    let t4158 = -0.35981577432354634426e-1 * t3935 * t3939 - 0.35981577432354634426e-1 * t1309 * t3944 + 0.17990788716177317213e-1 * t1309 * t3948 + 0.23987718288236422951e-1 * t1309 * t3955 + 0.10794473229706390328e0 * t1309 * t3963 + 0.35981577432354634426e-1 * t3966 * t1315 - 0.95950873152945691804e-1 * t3970 * t1315 + 0.11993859144118211475e-1 * t3975 + 0.28785261945883707542e0 * t3970 * t1324 - t3983 - 0.35981577432354634426e-1 * t3985 - 0.5397236614853195164e-1 * t1309 * t3990 - 0.28785261945883707542e0 * t3993 * t405 + 0.35981577432354634426e-1 * t3996 - 0.10794473229706390328e0 * t3966 * t1324 + 0.52772980234120130494e0 * t4001 * t405 - 0.95950873152945691804e-1 * t4004 + 0.5397236614853195164e-1 * t4155 * t405;
    (t4144, t4148, t4154, t4155, t4158)
}
