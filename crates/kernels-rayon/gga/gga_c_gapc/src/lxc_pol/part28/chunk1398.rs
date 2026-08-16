//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1398/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1398(t34421: f64, t34424: f64, t34426: f64, t34428: f64, t34436: f64, t34439: f64, t34442: f64, t34410: f64, t34413: f64, t34417: f64, t34433: f64, t34449: f64) -> (f64, f64) {
    let t36934 = 0.48917046440972222224e-4_f64 * t34421;
    let t36935 = 0.25002399603899953676e-2_f64 * t34424;
    let t36936 = 0.3243554543208642639e-2_f64 * t34426;
    let t36937 = 0.3243554543208642639e-2_f64 * t34428;
    let t36939 = 0.15006749152217248259e-7_f64 * t34436;
    let t36940 = 0.21720231316129303386e-4_f64 * t34439;
    let t36941 = 0.2318836277704281739e-4_f64 * t34442;
    let t36942 = 0.66297786877786731988e-7_f64 * t34410 + 0.98332751566569010434e-8_f64 * t34413 - 0.89048050908546122981e-5_f64 * t34417 - t36934 + t36935 + t36936 + t36937 - 0.44198524585191154658e-7_f64 * t34433 - t36939 + t36940 - t36941;
    let t36945 = 0.57920616843011475696e-5_f64 * t34449;
    (t36942, t36945)
}
