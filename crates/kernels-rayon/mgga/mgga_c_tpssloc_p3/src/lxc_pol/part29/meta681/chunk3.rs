//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2296/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2296(t27526: f64, t86094: f64, t24660: f64, t24850: f64, t1409: f64, t3507: f64, t24667: f64, t24847: f64, t64825: f64, t974: f64, t8067: f64, t85660: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94947 = 0.18277045187202515961e-2_f64 * t86094 * t27526;
    let t94948 = t24660 * t24850;
    let t94949 = t1409 * t3507;
    let t94954 = t24667 * t24850;
    let t94963 = t24847 * t974 * t64825;
    let t94966 = t85660 * t8067;
    (t94947, t94948, t94949, t94954, t94963, t94966)
}
