//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 620/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk620(t15284: f64, t15288: f64, t15292: f64, t3225: f64, t8368: f64, t22: f64, t2447: f64, t656: f64, t2145: f64, t15297: f64, t2265: f64, t2415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15609 = 0.68186654135613354325e-2_f64 * t15284;
    let t15610 = 0.68186654135613354325e-2_f64 * t15288;
    let t15611 = 0.20455996240684006296e-1_f64 * t15292;
    let t15614 = t8368 * t3225;
    let t15615 = 0.34093327067806677161e-2_f64 * t15614;
    let t15616 = t2447 * t22;
    let t15617 = t15616 * t656;
    let t15618 = t2145 * t15617;
    let t15619 = 0.34093327067806677161e-2_f64 * t15618;
    let t15620 = 0.1276937996798935182e-4_f64 * t15297;
    let t15621 = t2415 * t2265;
    (t15609, t15610, t15611, t15615, t15616, t15617, t15619, t15620, t15621)
}
