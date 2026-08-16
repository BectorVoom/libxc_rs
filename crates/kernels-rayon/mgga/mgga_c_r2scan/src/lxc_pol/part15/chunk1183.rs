//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1183/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1183(t1054: f64, t5108: f64, t7333: f64, t39613: f64, t40195: f64, t7356: f64, t6106: f64, t7322: f64, t10856: f64, t8071: f64, t37769: f64, t7620: f64) -> (f64, f64, f64, f64, f64) {
    let t40207 = t5108 * t1054 * t7333;
    let t40210 = t39613 * t40195 * t7356;
    let t40213 = t6106 * t1054 * t7322;
    let t40215 = t10856 * t8071;
    let t40216 = 0.97574405393827830186e-2_f64 * t40215;
    let t40217 = t37769 * t7620;
    (t40207, t40210, t40213, t40216, t40217)
}
