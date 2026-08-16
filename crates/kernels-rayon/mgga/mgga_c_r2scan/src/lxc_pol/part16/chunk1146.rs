//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1146/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1146(t322: f64, t42521: f64, t42546: f64, t1013: f64, t11063: f64, t11066: f64, t11897: f64, t2394: f64, t2400: f64, t2941: f64, t2944: f64, t327: f64, t3373: f64, t37020: f64, t37023: f64, t40764: f64, t40770: f64, t42478: f64, t829: f64, t834: f64, t9676: f64, t9687: f64, t9690: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t42547 = t42521 + t42546;
    let t42548 = piecewise3(t324, 0.0_f64, t42547);
    let t42559 = -0.128e1_f64 * t42478 * t829 - 0.256e1_f64 * t40764 * t1013 - 0.256e1_f64 * t11897 * t2394 - 0.384e1_f64 * t37020 * t2944 - 0.128e1_f64 * t11063 * t2941 - 0.128e1_f64 * t3373 * t9676 - 0.64e0_f64 * t834 * t42548 - 0.64e0_f64 * t42548 * t327 - 0.768e1_f64 * t40770 * t2400 - 0.768e1_f64 * t11066 * t9690 - 0.1536e2_f64 * t37023 * t9687;
    (t42547, t42559)
}
