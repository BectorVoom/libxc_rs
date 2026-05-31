//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1146/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1146<F: Float>(t322: F, t42521: F, t42546: F, t1013: F, t11063: F, t11066: F, t11897: F, t2394: F, t2400: F, t2941: F, t2944: F, t327: F, t3373: F, t37020: F, t37023: F, t40764: F, t40770: F, t42478: F, t829: F, t834: F, t9676: F, t9687: F, t9690: F) -> (F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t42547 = t42521 + t42546;
    let t42548 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t42547);
    let t42559 = -F::cast_from(0.128e1_f64) * t42478 * t829 - F::cast_from(0.256e1_f64) * t40764 * t1013 - F::cast_from(0.256e1_f64) * t11897 * t2394 - F::cast_from(0.384e1_f64) * t37020 * t2944 - F::cast_from(0.128e1_f64) * t11063 * t2941 - F::cast_from(0.128e1_f64) * t3373 * t9676 - F::cast_from(0.64e0_f64) * t834 * t42548 - F::cast_from(0.64e0_f64) * t42548 * t327 - F::cast_from(0.768e1_f64) * t40770 * t2400 - F::cast_from(0.768e1_f64) * t11066 * t9690 - F::cast_from(0.1536e2_f64) * t37023 * t9687;
    (t42547, t42559)
}
