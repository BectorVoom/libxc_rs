//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1009/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1009(t13810: f64, t4950: f64, t12068: f64, t1445: f64, t1562: f64, t2293: f64, t11986: f64, t2464: f64, t2465: f64, t587: f64, t48086: f64, t544: f64) -> (f64, f64, f64, f64) {
    let t48144 = t4950 * t13810;
    let t48149 = t1562 * t1445 * t12068 * t2293;
    let t48154 = t587 * t2464 * t2465 * t11986;
    let t48156 = t544 * t48086;
    (t48144, t48149, t48154, t48156)
}
