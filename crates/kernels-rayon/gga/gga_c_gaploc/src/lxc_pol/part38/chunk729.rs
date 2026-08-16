//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 729/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk729(t1683: f64, t5335: f64, t5344: f64, t92: f64, t291: f64, t293: f64, t5343: f64, t5747: f64, t801: f64, t6059: f64, t769: f64, t121: f64, t5745: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15665 = 1.0_f64 / t5335 / t1683;
    let t15667 = t15665 * t92 * t5344;
    let t15672 = t291 / t5343 / t293;
    let t15751 = t801 * t5747;
    let t15766 = t769 * t6059;
    let t16534 = t121 * t5745;
    (t15665, t15667, t15672, t15751, t15766, t16534)
}
