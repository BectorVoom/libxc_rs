//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1130/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1130(t19040: f64, t261: f64, t3005: f64, t6423: f64, t1226: f64, t15351: f64, t4763: f64, t18645: f64, t18661: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t18828: f64, t18830: f64, t18833: f64, t18835: f64, t18904: f64) -> (f64, f64, f64, f64) {
    let t19042 = 0.62182e-1_f64 * t19040 * t261;
    let t19043 = t3005 * t6423;
    let t19044 = t19043 * t1226;
    let t19047 = t4763 * t15351;
    let t19071 = 0.258925e1_f64 * t18835 + 0.19419375e1_f64 * t18828 - 0.258925e1_f64 * t18830 - 0.1294625e1_f64 * t18833 - 0.20128333333333333333e0_f64 * t18674 + 0.60385e0_f64 * t18679 + 0.67094444444444444443e-1_f64 * t18645 - 0.20128333333333333333e0_f64 * t18661 + 0.10064166666666666667e0_f64 * t18669 - 0.301925e0_f64 * t18683 + 0.16557e0_f64 * t18904;
    (t19042, t19044, t19047, t19071)
}
