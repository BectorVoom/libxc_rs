//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1140/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1140<F: Float>(t3579: F, t40590: F, t10610: F, t11479: F, t11509: F, t12574: F, t792: F, t3275: F, t37299: F, t12602: F, t833: F, t23495: F, t3629: F) -> (F, F, F, F, F) {
    let t42467 = F::new(5.0) / F::new(8.0) * t3579 * t40590;
    let t42471 = F::new(3.0) * t10610 * t11479 * t11509;
    let t42472 = t12574 * t792;
    let t42475 = F::new(585.0) / F::new(256.0) * t3275 * t37299 * t42472;
    let t42478 = t12602 * t833;
    let t42491 = t23495 * t3629;
    (t42467, t42471, t42475, t42478, t42491)
}
