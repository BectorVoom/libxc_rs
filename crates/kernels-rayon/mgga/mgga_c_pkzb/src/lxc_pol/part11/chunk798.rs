//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 798/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk798(t1196: f64, t6290: f64, t2320: f64, t3135: f64, t1208: f64, t6233: f64, t1184: f64, t6201: f64, t1189: f64, t2256: f64, t3030: f64, t832: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8153 = t1196 * t6290;
    let t8170 = t3135 * t2320;
    let t8177 = t1208 * t6233;
    let t8205 = t1184 * t6201;
    let t8211 = t1189 * t2256;
    let t8214 = t3030 * t832;
    (t8153, t8170, t8177, t8205, t8211, t8214)
}
