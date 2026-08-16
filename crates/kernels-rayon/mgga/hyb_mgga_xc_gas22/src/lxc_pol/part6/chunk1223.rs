//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1223/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1223(t191: f64, t240: f64, t6452: f64, t8511: f64, t8514: f64, t2026: f64, t6610: f64, t3138: f64, t8521: f64, t763: f64, t8512: f64, t8518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23889 = t240 * t6452 * t191;
    let t23891 = t8511 * t23889 * t8514;
    let t23894 = t6610 * t2026 * t191;
    let t23896 = t3138 * t23894 * t8521;
    let t23905 = t8512 * t763;
    let t23909 = t8518 * t763;
    (t23889, t23891, t23894, t23896, t23905, t23909)
}
