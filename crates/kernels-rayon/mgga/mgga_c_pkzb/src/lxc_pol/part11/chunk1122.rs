//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1122/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1122(t23388: f64, t411: f64, t6546: f64, t2363: f64, t3246: f64, t2393: f64, t1448: f64, t3308: f64, t1435: f64, t980: f64, t991: f64, t1625: f64, t3380: f64, t83: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23389 = 0.14291339372689912324e-3_f64 * t23388;
    let t23398 = t411 * t6546;
    let t23465 = t2363 * t3246;
    let t23472 = t2393 * t3246;
    let t23711 = t3308 * t1448;
    let t23796 = t980 * t1435;
    let t23870 = t991 * t1435;
    let t23943 = t83 * t3380 * t1625;
    (t23389, t23398, t23465, t23472, t23711, t23796, t23870, t23943)
}
