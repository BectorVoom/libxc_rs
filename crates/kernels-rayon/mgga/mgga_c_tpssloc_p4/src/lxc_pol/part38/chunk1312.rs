//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1312/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1312(t1266: f64, t8230: f64, t1849: f64, t8143: f64, t30180: f64, t510: f64, t2180: f64, t5107: f64, t1393: f64, t1268: f64, t12725: f64, t19456: f64, t2181: f64, t2183: f64, t2314: f64, t26114: f64, t26117: f64, t4028: f64, t4034: f64, t652: f64, t7458: f64, t8124: f64, t8144: f64, t8148: f64, t8221: f64, t8231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30195 = t1266 * t8230;
    let t30201 = t8143 * t1849;
    let t30203 = t510 * t30180;
    let t30209 = t5107 * t2180;
    let t30211 = t8230 * t1393;
    let t30215 = t1268 * t30201 + t1268 * t30211 + t12725 * t2183 - t19456 * t2181 + t19456 * t2183 + t2183 * t26114 + t2183 * t26117 - t2314 * t8221 - t30195 * t652 - t30203 * t652 - t30209 * t652 - t4028 * t8124 + t4028 * t8148 - t4034 * t8221 - t4034 * t8231 - t7458 * t8144;
    (t30195, t30201, t30203, t30209, t30211, t30215)
}
