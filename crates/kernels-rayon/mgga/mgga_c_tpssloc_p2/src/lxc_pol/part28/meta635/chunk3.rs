//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2014/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2014(t90844: f64, t90859: f64, t90864: f64, t90866: f64, t90868: f64, t1332: f64, t1336: f64, t16047: f64, t16048: f64, t16055: f64, t24117: f64, t24131: f64, t27074: f64, t27075: f64, t27097: f64, t27105: f64, t3793: f64, t3856: f64, t5234: f64, t5334: f64, t81022: f64, t90848: f64, t90852: f64, t90856: f64, t90873: f64) -> f64 {
    let t93524 = 0.3289868133696452873e-1_f64 * t90844;
    let t93528 = 0.16449340668482264365e-1_f64 * t90859;
    let t93529 = 0.16449340668482264365e-1_f64 * t90864;
    let t93537 = 0.76763589786250567036e-1_f64 * t90866;
    let t93538 = 0.12793931631041761173e0_f64 * t90868;
    let t93546 = -0.16449340668482264365e-1_f64 * t81022 + 4.0_f64 * t16055 * t27075 - t93524 + 0.6579736267392905746e-1_f64 * t90848 - 0.16449340668482264365e-1_f64 * t90852 + 0.9869604401089358619e-1_f64 * t90856 + t93528 + t93529 - 6.0_f64 * t16047 * t27074 * t16048 + 6.0_f64 * t5334 * t27074 * t3793 - t5234 * t24131 - t93537 + t93538 + 2.0_f64 * t1332 * t27105 - t1336 * t27097 * t3856 - 2.0_f64 * t5234 * t24117 - 0.16449340668482264365e-1_f64 * t90873;
    t93546
}
