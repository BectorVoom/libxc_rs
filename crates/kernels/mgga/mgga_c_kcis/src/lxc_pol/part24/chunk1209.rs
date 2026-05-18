//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1209/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1209<F: Float>(t27070: F, t28093: F, t96395: F, t96401: F, t96427: F, t1281: F, t28250: F, t4527: F, t7671: F, t1655: F, t26654: F, t27759: F) -> (F, F, F, F, F, F, F, F) {
    let t97431 = F::new(0.30918233506944444444e-4) * t27070 * t28093;
    let t97442 = F::new(0.10317654320987654321e-2) * t96395;
    let t97449 = F::new(0.15476481481481481481e-2) * t96401;
    let t97465 = F::new(0.23214722222222222222e-2) * t96427;
    let t97494 = t28250 * t1281;
    let t97561 = F::new(2.0) * t4527 * t7671;
    let t97601 = t1655 * t26654;
    let t97606 = t27759 / F::new(8.0);
    (t97431, t97442, t97449, t97465, t97494, t97561, t97601, t97606)
}
