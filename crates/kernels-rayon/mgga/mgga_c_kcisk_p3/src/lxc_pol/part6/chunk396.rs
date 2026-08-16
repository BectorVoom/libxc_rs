//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 396/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk396(t1908: f64, t2594: f64, t1959: f64, t2366: f64, t1968: f64, t1971: f64, t2373: f64, t2376: f64, t2379: f64, t1974: f64, t1685: f64, t2394: f64) -> (f64, f64, f64, f64, f64) {
    let t2595 = t1908 * t2594;
    let t2597 = -t1959 - 0.17123333333333333333e-1_f64 * t2366;
    let t2604 = 0.3529725e1_f64 * t2373 - t1968 - 0.516475e0_f64 * t2366 + 0.6311625e0_f64 * t2376 - t1971 - 0.104195e0_f64 * t2379;
    let t2605 = t2604 * t1974;
    let t2609 = t2394 * t1685;
    (t2595, t2597, t2604, t2605, t2609)
}
