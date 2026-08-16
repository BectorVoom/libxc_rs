//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1326/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326(t39838: f64, t39853: f64, t162: f64, t187: f64, t10428: f64, t2615: f64, t2622: f64, t9586: f64, t2514: f64, t2492: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39854 = t39838 + t39853;
    let t39857 = 0.19751673498613801407e-1_f64 * t39854 * t162 * t187;
    let t39858 = t10428 * t2615;
    let t39859 = 48.0_f64 * t39858;
    let t39860 = t2622 * t9586;
    let t39861 = 0.22787578869697033845e-2_f64 * t39860;
    let t39871 = t2514 * t2514;
    let t39875 = t2492 * t2492;
    (t39854, t39857, t39859, t39861, t39871, t39875)
}
