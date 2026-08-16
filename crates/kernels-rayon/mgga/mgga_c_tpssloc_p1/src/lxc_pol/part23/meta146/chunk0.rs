//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 688/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk688(t349: f64, t5914: f64, t1634: f64, t3174: f64, t381: f64, t5872: f64, t3188: f64, t1615: f64, t1625: f64, t1060: f64, t5866: f64, t3201: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5915 = t349 * t5914;
    let t5919 = t1634 * t1634;
    let t5920 = t3174 * t5919;
    let t5928 = t381 * t5872;
    let t5929 = t5928 * t3188;
    let t5932 = t1625 * t1615;
    let t5933 = t5932 * t1060;
    let t5936 = t381 * t5866;
    let t5937 = t5936 * t1060;
    let t5939 = t5928 * t3201;
    (t5915, t5919, t5920, t5928, t5929, t5933, t5936, t5937, t5939)
}
