//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1257/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1257(t15878: f64, t21003: f64, t4160: f64, t21002: f64, t5426: f64, t5661: f64, t12286: f64, t15844: f64, t20970: f64, t20977: f64, t20982: f64, t20987: f64, t20991: f64, t20996: f64, t21000: f64, t7043: f64) -> (f64, f64, f64) {
    let t21004 = t15878 * t21003;
    let t21005 = t4160 * t21004;
    let t21007 = t5426 * t21002;
    let t21008 = t15878 * t21007;
    let t21009 = t5661 * t21008;
    let t21011 = -0.11054629629629629629e-2_f64 * t20970 + 0.890445125e-2_f64 * t12286 * t7043 + 0.33163888888888888888e-2_f64 * t20977 - 0.22109259259259259259e-2_f64 * t20982 + 0.99491666666666666664e-2_f64 * t20987 + 0.13265555555555555555e-1_f64 * t20991 + 0.22109259259259259259e-2_f64 * t15844 + 0.66327777777777777776e-2_f64 * t20996 - 0.22109259259259259259e-2_f64 * t21000 + 0.66327777777777777776e-2_f64 * t21005 - 0.55273148148148148147e-2_f64 * t21009;
    (t21005, t21009, t21011)
}
