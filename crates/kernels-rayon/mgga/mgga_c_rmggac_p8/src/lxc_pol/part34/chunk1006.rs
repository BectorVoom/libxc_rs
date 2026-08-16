//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1006/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1006(t1525: f64, t1971: f64, t515: f64, t698: f64, t7230: f64, t15504: f64, t16043: f64, t637: f64, t8625: f64, t71163: f64, t8631: f64, t72142: f64) -> (f64, f64, f64, f64) {
    let t77572 = t7230 * t1971 * t515 * t698 * t1525;
    let t77573 = 0.53205749866622299248e-5_f64 * t77572;
    let t77574 = t16043 * t15504;
    let t77575 = 0.42564599893297839398e-5_f64 * t77574;
    let t77576 = t637 * t8625;
    let t77577 = t71163 * t77576;
    let t77578 = 0.40911992481368012592e-1_f64 * t77577;
    let t77579 = t637 * t8631;
    let t77580 = t72142 * t77579;
    (t77573, t77575, t77578, t77580)
}
