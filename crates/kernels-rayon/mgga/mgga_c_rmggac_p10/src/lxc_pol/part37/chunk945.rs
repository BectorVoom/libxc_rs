//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 945/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk945(t77116: f64, t14683: f64, t8577: f64, t71021: f64, t68742: f64, t3219: f64, t38351: f64, t38355: f64, t14639: f64, t8571: f64, t15457: f64, t16156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77117 = 0.90915538847484472429e-2_f64 * t77116;
    let t77118 = t8577 * t14683;
    let t77119 = 0.42564599893297839398e-5_f64 * t77118;
    let t77121 = 0.21684485328539747656e-4_f64 * t71021;
    let t77123 = 0.79828278012425390427e-1_f64 * t68742;
    let t77124 = t38351 * t3219;
    let t77125 = 0.42564599893297839398e-5_f64 * t77124;
    let t77126 = t38355 * t3219;
    let t77127 = 0.42564599893297839398e-5_f64 * t77126;
    let t77128 = t8571 * t14639;
    let t77129 = 0.42564599893297839398e-5_f64 * t77128;
    let t77131 = t16156 * t15457;
    (t77117, t77119, t77121, t77123, t77125, t77127, t77129, t77131)
}
