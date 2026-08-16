//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1346/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1346(t11365: f64, t51898: f64, t12215: f64, t3965: f64, t15317: f64, t51682: f64, t2409: f64, t36089: f64, t3959: f64, t14001: f64, t15331: f64, t1178: f64, t12169: f64, t371: f64, t3983: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57685 = t51898 * t11365;
    let t57687 = t3965 * t12215;
    let t57689 = t51682 * t15317;
    let t57694 = t3959 * t2409 * t36089;
    let t57696 = t14001 * t15331;
    let t57700 = t3983 * t371 * t1178 * t12169;
    (t57685, t57687, t57689, t57694, t57696, t57700)
}
