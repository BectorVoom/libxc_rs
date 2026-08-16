//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 623/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk623(t15319: f64, t15323: f64, t15326: f64, t15334: f64, t15359: f64, t15372: f64, t15395: f64, t15406: f64, t2211: f64, t8975: f64, t739: f64, t8946: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15644 = 0.3830813990396805546e-4_f64 * t15319;
    let t15645 = 0.1276937996798935182e-4_f64 * t15323;
    let t15646 = 0.1276937996798935182e-4_f64 * t15326;
    let t15648 = 0.2627895913935205078e-5_f64 * t15334;
    let t15656 = 0.14967802127329760705e-1_f64 * t15359;
    let t15660 = 0.23268647941669485538e-4_f64 * t15372;
    let t15665 = 0.1276937996798935182e-4_f64 * t15395;
    let t15667 = 0.85129199786595678799e-5_f64 * t15406;
    let t15669 = t2211 * t8975;
    let t15670 = t739 * t15669;
    let t15671 = 0.11974241701863808564e0_f64 * t15670;
    let t15672 = t2211 * t8946;
    (t15644, t15645, t15646, t15648, t15656, t15660, t15665, t15667, t15669, t15671, t15672)
}
