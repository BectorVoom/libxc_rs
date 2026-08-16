//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1158/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1158(t10772: f64, t3308: f64, t9135: f64, t10776: f64, t9139: f64, t9143: f64, t3295: f64, t9526: f64, t27067: f64, t3610: f64, t29274: f64, t3332: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43048 = t10772 * t3308 * t9135;
    let t43051 = t10776 * t3308 * t9139;
    let t43054 = t10772 * t3308 * t9143;
    let t43057 = t3295 * t9526;
    let t43061 = t27067 * t3610;
    let t43072 = t7614 * t3332 * t29274;
    (t43048, t43051, t43054, t43057, t43061, t43072)
}
