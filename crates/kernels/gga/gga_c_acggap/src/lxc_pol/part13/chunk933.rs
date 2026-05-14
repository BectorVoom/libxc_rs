//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 933/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk933<F: Float>(t34398: F, t1352: F, t7736: F, t2450: F, t31349: F, t4469: F, t7839: F, t8481: F, t30463: F, t30469: F, t30497: F, t30507: F, t30511: F, t30522: F, t30524: F, t34383: F, t34385: F, t34388: F, t34391: F, t34392: F, t34394: F, t34396: F) -> (F, F) {
    let t34399 = 0.56606566121287473722e-2 * t34398;
    let t34400 = t7736 * t1352;
    let t34406 = t2450 * t31349;
    let t34407 = t34406 * t4469;
    let t34409 = t7839 * t8481;
    let t34410 = 0.21437009059034868486e-3 * t34409;
    let t34412 = -0.94344276868812456204e-3 * t30463 + 0.34299214494455789578e-2 * t30469 + t34383 - 5.0 / 32.0 * t34385 - t34388 / 64.0 - t34391 + 0.13073958333333333333e0 * t34392 - 0.21437009059034868486e-3 * t34394 + 0.80031500487063509014e-2 * t34396 - t34399 - 0.17149607247227894789e-2 * t34400 - 0.37737710747524982482e-2 * t30497 + 0.56606566121287473722e-2 * t30507 + 0.10718504529517434243e-3 * t30511 - 0.94344276868812456204e-3 * t30522 - 0.34299214494455789578e-1 * t34407 - t34410 + 0.64311027177104605458e-3 * t30524;
    (t34406, t34412)
}
