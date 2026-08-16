//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 284/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk284(t2211: f64, t352: f64, t118: f64, t2098: f64, t2107: f64, t2113: f64, t2121: f64, t2094: f64, t2096: f64, t2101: f64, t2104: f64, t2109: f64, t2111: f64, t2116: f64, t2119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2212 = t2211 * t352;
    let t2213 = t118 * t2212;
    let t2217 = 0.26609426004141796809e-1_f64 * t2098;
    let t2220 = 0.60610359231656314955e-2_f64 * t2107;
    let t2223 = 0.35403077613494883571e-3_f64 * t2113;
    let t2226 = 0.80640343452960568133e-4_f64 * t2121;
    let t2227 = -0.19957069503106347607e-1_f64 * t2094 + 0.2993560425465952141e-1_f64 * t2096 + t2217 + 0.68186654135613354324e-2_f64 * t2101 - 0.90915538847484472432e-2_f64 * t2104 - t2220 - 0.66380770525302906695e-3_f64 * t2109 + 0.79656924630363488034e-3_f64 * t2111 + t2223 + 0.1814407727691612783e-3_f64 * t2116 - 0.21168090156402149135e-3_f64 * t2119 - t2226;
    (t2213, t2217, t2220, t2223, t2226, t2227)
}
