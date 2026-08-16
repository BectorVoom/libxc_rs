//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2025/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2025(t87066: f64, t25245: f64, t82031: f64, t25251: f64, t87049: f64, t23012: f64, t7529: f64, t23110: f64, t23185: f64, t25241: f64, t1484: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87067 = 0.38381794893125283518e-1_f64 * t87066;
    let t87068 = t82031 * t25245;
    let t87078 = t87049 * t25251;
    let t87080 = t23012 * t7529;
    let t87100 = t23185 * t23110 * t25241;
    let t87101 = 0.82246703342411321824e-2_f64 * t87100;
    let t87111 = t852 * t1484;
    (t87067, t87068, t87078, t87080, t87101, t87111)
}
