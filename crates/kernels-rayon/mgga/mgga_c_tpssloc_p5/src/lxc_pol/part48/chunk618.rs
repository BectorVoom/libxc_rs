//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 618/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk618(t1251: f64, t7301: f64, t7300: f64, t1190: f64, t2144: f64, t1193: f64, t2127: f64, t210: f64, t2120: f64, t2132: f64, t52: f64, t2136: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7302 = t7301 * t1251;
    let t7303 = t7300 * t7302;
    let t7306 = t1190 * t2144;
    let t7309 = t2127 * t1193 / 288.0_f64;
    let t7310 = t2120 * t210;
    let t7313 = t2132 * t52;
    let t7315 = 0.10093189023535097714e-3_f64 * t7313 * t2136;
    (t7302, t7303, t7306, t7309, t7310, t7315)
}
