//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1048/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1048<F: Float>(t1352: F, t7746: F, t1967: F, t8486: F, t7736: F, t2450: F, t31349: F, t4469: F, t7839: F, t8481: F, t30463: F, t30469: F, t30497: F, t30507: F, t30511: F, t30522: F, t30524: F, t34383: F, t34385: F, t34388: F, t34391: F, t34392: F, t34394: F) -> (F, F) {
    let t34396 = t7746 * t1352;
    let t34398 = t1967 * t8486;
    let t34399 = F::cast_from(0.56606566121287473722e-2_f64) * t34398;
    let t34400 = t7736 * t1352;
    let t34406 = t2450 * t31349;
    let t34407 = t34406 * t4469;
    let t34409 = t7839 * t8481;
    let t34410 = F::cast_from(0.21437009059034868486e-3_f64) * t34409;
    let t34412 = -F::cast_from(0.94344276868812456204e-3_f64) * t30463 + F::cast_from(0.34299214494455789578e-2_f64) * t30469 + t34383 - F::new(5.0) / F::new(32.0) * t34385 - t34388 / F::new(64.0) - t34391 + F::cast_from(0.13073958333333333333e0_f64) * t34392 - F::cast_from(0.21437009059034868486e-3_f64) * t34394 + F::cast_from(0.80031500487063509014e-2_f64) * t34396 - t34399 - F::cast_from(0.17149607247227894789e-2_f64) * t34400 - F::cast_from(0.37737710747524982482e-2_f64) * t30497 + F::cast_from(0.56606566121287473722e-2_f64) * t30507 + F::cast_from(0.10718504529517434243e-3_f64) * t30511 - F::cast_from(0.94344276868812456204e-3_f64) * t30522 - F::cast_from(0.34299214494455789578e-1_f64) * t34407 - t34410 + F::cast_from(0.64311027177104605458e-3_f64) * t30524;
    (t34406, t34412)
}
