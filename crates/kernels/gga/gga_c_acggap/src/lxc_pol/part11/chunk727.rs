//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 727/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk727<F: Float>(t1092: F, t2001: F, t1098: F, t2118: F, t957: F, t1089: F, t368: F, t7554: F, t7553: F, t2037: F, t7309: F, t7622: F, t7625: F, t7626: F, t7629: F, t7632: F, t7639: F, t7641: F, t7645: F, t7649: F, t7651: F, t7653: F, t7655: F, t7659: F, t7661: F) -> (F, F, F, F) {
    let t7663 = t2001 * t1092;
    let t7665 = t2001 * t1098;
    let t7667 = t2118 * t957;
    let t7670 = t1089 * t368 * t7554;
    let t7671 = t7553 * t7670;
    let t7672 = F::cast_from(0.21437009059034868486e-3_f64) * t7671;
    let t7673 = t7309 * t2037;
    let t7674 = F::cast_from(13.0_f64) / F::cast_from(288.0_f64) * t7673;
    let t7675 = F::cast_from(0.80031500487063509015e-2_f64) * t7622 - t7625 - F::cast_from(0.17149607247227894789e-2_f64) * t7626 + t7629 + t7632 + t7639 - t7641 + t7645 + t7649 + t7651 - t7653 + t7655 - F::cast_from(0.47172138434406228102e-2_f64) * t7659 - F::cast_from(0.34299214494455789578e-2_f64) * t7661 - F::cast_from(0.68598428988911579156e-2_f64) * t7663 + F::cast_from(0.68598428988911579156e-2_f64) * t7665 - F::cast_from(0.42874018118069736972e-3_f64) * t7667 + t7672 - t7674;
    (t7670, t7672, t7674, t7675)
}
