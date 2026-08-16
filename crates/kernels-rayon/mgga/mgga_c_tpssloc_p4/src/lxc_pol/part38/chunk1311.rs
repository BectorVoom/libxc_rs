//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1311/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1311(t30180: f64, t574: f64, t2180: f64, t5361: f64, t1774: f64, t8143: f64, t1268: f64, t12725: f64, t2181: f64, t2314: f64, t26114: f64, t26179: f64, t4028: f64, t5113: f64, t652: f64, t7458: f64, t7676: f64, t8124: f64, t8144: f64, t8148: f64, t8150: f64, t8231: f64, t8235: f64, t8237: f64) -> (f64, f64, f64, f64) {
    let t30181 = t30180 * t574;
    let t30186 = t2180 * t5361;
    let t30189 = t1774 * t8143;
    let t30192 = t1268 * t30181 + t1268 * t30186 - t12725 * t2181 - t2181 * t26114 - t2181 * t26179 - t2314 * t8231 + t2314 * t8235 + t2314 * t8237 - t30189 * t652 - t4028 * t8144 + t4028 * t8150 + t5113 * t8235 + t5113 * t8237 - t7458 * t8124 + t7676 * t8148 + t7676 * t8150;
    (t30181, t30186, t30189, t30192)
}
