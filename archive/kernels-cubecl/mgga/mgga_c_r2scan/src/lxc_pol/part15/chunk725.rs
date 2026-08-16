//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 725/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk725<F: Float>(t672: F, t685: F, t63: F, t1664: F, t1800: F, t649: F, t2029: F, t689: F, t1663: F, t5381: F, t390: F, t188: F, t1890: F) -> (F, F, F, F, F) {
    let t5747 = F::cast_from(1.0_f64) / t685 / t672;
    let t5748 = t63 * t5747;
    let t5754 = F::cast_from(0.10310157056611784231e2_f64) * t649 * t1800 * t1664;
    let t5755 = t2029 * t689;
    let t5759 = t1663 * t5381;
    let t5761 = F::cast_from(0.85917975471764868594e0_f64) * t390 * t5759;
    let t5762 = t1890 * t188;
    (t5748, t5754, t5755, t5761, t5762)
}
