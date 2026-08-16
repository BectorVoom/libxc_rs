//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 585/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk585<F: Float>(t1196: F, t5202: F, t1756: F, t3520: F, t1187: F, t3523: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F) -> (F, F, F) {
    let t5204 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t5202;
    let t5205 = t3520 * t1756;
    let t5206 = t3523 * t1187;
    let t5207 = t5205 * t5206;
    let t5209 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t5207;
    let t5215 = t3546 - F::cast_from(0.27777777777777777778e-2_f64) * t3358 - F::cast_from(0.27777777777777777778e-2_f64) * t5044 - F::cast_from(0.55555555555555555555e-2_f64) * t5049 + F::cast_from(0.16666666666666666667e-1_f64) * t5054 + F::cast_from(0.83333333333333333333e-2_f64) * t5058;
    (t5204, t5209, t5215)
}
