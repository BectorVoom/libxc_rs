//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 947/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk947(t1036: f64, t5121: f64, t11488: f64, t1688: f64, t5126: f64, t11320: f64, t185: f64, t1697: f64, t3122: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11489 = t1036 * t5121;
    let t11490 = t11488 * t11489;
    let t11492 = t1688 * t5126;
    let t11493 = t11488 * t11492;
    let t11495 = t185 * t11320;
    let t11496 = t1697 * t3122;
    (t11489, t11490, t11492, t11493, t11495, t11496)
}
