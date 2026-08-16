//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1222/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1222(t13714: f64, t13717: f64, t13912: f64, t15397: f64, t15411: f64, t18650: f64, t18655: f64, t18659: f64, t18664: f64, t18667: f64, t18877: f64, t18880: f64, t18885: f64, t18887: f64, t18890: f64, t18909: f64, t18912: f64, t18920: f64, t20431: f64, t20452: f64, t9691: f64, t9708: f64) -> f64 {
    let t20454 = -0.22954444444444444444e0_f64 * t9691 - 0.11577222222222222222e0_f64 * t9708 + 0.23154444444444444445e-1_f64 * t18877 - 0.104195e0_f64 * t18880 - t15397 + 0.4630888888888888889e-1_f64 * t13912 + 0.68863333333333333332e0_f64 * t13717 - 0.157790625e0_f64 * t18885 + 0.6311625e0_f64 * t18887 + 0.31558125e0_f64 * t18890 + t20431 - 0.46308888888888888889e-1_f64 * t18909 - 0.13892666666666666667e0_f64 * t18912 + t15411 - 0.68863333333333333332e0_f64 * t13714 - 0.57386111111111111112e0_f64 * t18650 + 0.20659e1_f64 * t18655 - 0.13772666666666666667e1_f64 * t18659 - 0.309885e1_f64 * t18664 + 0.41318e1_f64 * t18667 + 0.6311625e0_f64 * t18920 + t20452;
    t20454
}
