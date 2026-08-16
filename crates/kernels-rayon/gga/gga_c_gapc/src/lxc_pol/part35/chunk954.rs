//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 954/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk954(t11643: f64, t3734: f64, t10286: f64, t11270: f64, t2923: f64, t7108: f64, t959: f64, t3225: f64, t3729: f64, t828: f64, t11614: f64, t11617: f64, t11621: f64, t11623: f64, t11627: f64, t11630: f64, t11634: f64, t11638: f64, t11641: f64) -> (f64, f64) {
    let t11644 = t11643 * t3734;
    let t11646 = t11270 * t10286;
    let t11648 = t2923 * t959 * t7108;
    let t11649 = t11646 * t11648;
    let t11651 = t3225 * t3734;
    let t11653 = t828 * t3729;
    let t11655 = -0.82073827867876094584e-5_f64 * t11614 - 0.82073827867876094584e-5_f64 * t11617 + 0.11742981196020707897e-5_f64 * t11621 - 0.80732995722642366792e-5_f64 * t11623 + 0.11742981196020707897e-4_f64 * t11627 + 0.73393632475129424356e-6_f64 * t11630 + 0.43497959513593372169e-7_f64 * t11634 + 0.73393632475129424356e-6_f64 * t11638 + 0.11742981196020707897e-4_f64 * t11641 - 0.17098714139140853038e-6_f64 * t11644 - 0.49871249572494154694e-7_f64 * t11649 + 0.15388842725226767735e-5_f64 * t11651 + 0.46971924784082831588e-4_f64 * t11653;
    (t11648, t11655)
}
