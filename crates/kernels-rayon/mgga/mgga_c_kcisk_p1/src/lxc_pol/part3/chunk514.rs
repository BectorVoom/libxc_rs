//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 514/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk514(t385: f64, t3777: f64, t4143: f64, t1284: f64, t3502: f64, t1280: f64, t1287: f64, t340: f64, t379: f64, t382: f64, t4134: f64, t395: f64, t1309: f64, t1315: f64, t1324: f64, t3935: f64, t3939: f64, t3944: f64, t3948: f64, t3955: f64, t3963: f64, t3966: f64, t3970: f64, t3975: f64, t3983: f64, t3985: f64, t3990: f64, t3993: f64, t3996: f64, t4001: f64, t4004: f64, t405: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t386 = t385 < -0.66725e-1_f64;
    let t4144 = t4143 * t3777;
    let t4148 = t1284 * t3502;
    let t4153 = piecewise3(t386, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t4134 * t382 - 20.0_f64 / 27.0_f64 * t340 * t1280 * t1287 + 40.0_f64 / 81.0_f64 * t340 * t379 * t4144 - 10.0_f64 / 27.0_f64 * t340 * t379 * t4148);
    let t4154 = t4153 * sigma0;
    let t4155 = t4154 * t395;
    let t4158 = -0.35981577432354634426e-1_f64 * t3935 * t3939 - 0.35981577432354634426e-1_f64 * t1309 * t3944 + 0.17990788716177317213e-1_f64 * t1309 * t3948 + 0.23987718288236422951e-1_f64 * t1309 * t3955 + 0.10794473229706390328e0_f64 * t1309 * t3963 + 0.35981577432354634426e-1_f64 * t3966 * t1315 - 0.95950873152945691804e-1_f64 * t3970 * t1315 + 0.11993859144118211475e-1_f64 * t3975 + 0.28785261945883707542e0_f64 * t3970 * t1324 - t3983 - 0.35981577432354634426e-1_f64 * t3985 - 0.5397236614853195164e-1_f64 * t1309 * t3990 - 0.28785261945883707542e0_f64 * t3993 * t405 + 0.35981577432354634426e-1_f64 * t3996 - 0.10794473229706390328e0_f64 * t3966 * t1324 + 0.52772980234120130494e0_f64 * t4001 * t405 - 0.95950873152945691804e-1_f64 * t4004 + 0.5397236614853195164e-1_f64 * t4155 * t405;
    (t4144, t4148, t4154, t4155, t4158)
}
