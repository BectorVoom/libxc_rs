//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1048/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1048(t1352: f64, t7746: f64, t1967: f64, t8486: f64, t7736: f64, t2450: f64, t31349: f64, t4469: f64, t7839: f64, t8481: f64, t30463: f64, t30469: f64, t30497: f64, t30507: f64, t30511: f64, t30522: f64, t30524: f64, t34383: f64, t34385: f64, t34388: f64, t34391: f64, t34392: f64, t34394: f64) -> (f64, f64) {
    let t34396 = t7746 * t1352;
    let t34398 = t1967 * t8486;
    let t34399 = 0.56606566121287473722e-2_f64 * t34398;
    let t34400 = t7736 * t1352;
    let t34406 = t2450 * t31349;
    let t34407 = t34406 * t4469;
    let t34409 = t7839 * t8481;
    let t34410 = 0.21437009059034868486e-3_f64 * t34409;
    let t34412 = -0.94344276868812456204e-3_f64 * t30463 + 0.34299214494455789578e-2_f64 * t30469 + t34383 - 5.0_f64 / 32.0_f64 * t34385 - t34388 / 64.0_f64 - t34391 + 0.13073958333333333333e0_f64 * t34392 - 0.21437009059034868486e-3_f64 * t34394 + 0.80031500487063509014e-2_f64 * t34396 - t34399 - 0.17149607247227894789e-2_f64 * t34400 - 0.37737710747524982482e-2_f64 * t30497 + 0.56606566121287473722e-2_f64 * t30507 + 0.10718504529517434243e-3_f64 * t30511 - 0.94344276868812456204e-3_f64 * t30522 - 0.34299214494455789578e-1_f64 * t34407 - t34410 + 0.64311027177104605458e-3_f64 * t30524;
    (t34406, t34412)
}
