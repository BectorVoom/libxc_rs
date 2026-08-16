//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 823/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk823(t2294: f64, t2578: f64, t2139: f64, t1570: f64, t2567: f64, t360: f64, t1551: f64, t2124: f64, t2545: f64, t2553: f64, t6118: f64, t113: f64, t1543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7365 = t2294 * t2578;
    let t7367 = 0.69345773920434148506e0_f64 * t2139 * t7365;
    let t7368 = t2567 * t1570;
    let t7369 = t360 * t7368;
    let t7373 = t2124 * t2545 * t1551;
    let t7377 = 0.25610080155860322884e0_f64 * t6118 * t2553;
    let t7378 = t113 * t1543;
    (t7367, t7368, t7369, t7373, t7377, t7378)
}
