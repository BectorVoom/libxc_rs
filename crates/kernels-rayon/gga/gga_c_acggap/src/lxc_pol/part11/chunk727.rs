//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 727/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk727(t1092: f64, t2001: f64, t1098: f64, t2118: f64, t957: f64, t1089: f64, t368: f64, t7554: f64, t7553: f64, t2037: f64, t7309: f64, t7622: f64, t7625: f64, t7626: f64, t7629: f64, t7632: f64, t7639: f64, t7641: f64, t7645: f64, t7649: f64, t7651: f64, t7653: f64, t7655: f64, t7659: f64, t7661: f64) -> (f64, f64, f64, f64) {
    let t7663 = t2001 * t1092;
    let t7665 = t2001 * t1098;
    let t7667 = t2118 * t957;
    let t7670 = t1089 * t368 * t7554;
    let t7671 = t7553 * t7670;
    let t7672 = 0.21437009059034868486e-3_f64 * t7671;
    let t7673 = t7309 * t2037;
    let t7674 = 13.0_f64 / 288.0_f64 * t7673;
    let t7675 = 0.80031500487063509015e-2_f64 * t7622 - t7625 - 0.17149607247227894789e-2_f64 * t7626 + t7629 + t7632 + t7639 - t7641 + t7645 + t7649 + t7651 - t7653 + t7655 - 0.47172138434406228102e-2_f64 * t7659 - 0.34299214494455789578e-2_f64 * t7661 - 0.68598428988911579156e-2_f64 * t7663 + 0.68598428988911579156e-2_f64 * t7665 - 0.42874018118069736972e-3_f64 * t7667 + t7672 - t7674;
    (t7670, t7672, t7674, t7675)
}
