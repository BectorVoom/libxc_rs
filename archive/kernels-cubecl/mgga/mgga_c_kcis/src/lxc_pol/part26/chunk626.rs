//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 626/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk626<F: Float>(t1356: F, t3918: F, t7002: F, t3926: F, t3933: F, t5469: F, t5562: F, t6939: F, t6942: F, t6946: F, t6958: F, t6965: F, t6971: F, t6973: F, t6977: F, t6980: F, t6983: F) -> (F, F) {
    let t7004 = t3918 * t7002 * t1356;
    let t7019 = -F::cast_from(0.1294625e1_f64) * t6958 + F::cast_from(0.258925e1_f64) * t6965 + t3926 + F::cast_from(0.20128333333333333334e0_f64) * t5469 - F::cast_from(0.20128333333333333333e0_f64) * t6939 + F::cast_from(0.60385e0_f64) * t6942 - F::cast_from(0.301925e0_f64) * t6946 + F::cast_from(0.82524375e-1_f64) * t6971 + F::cast_from(0.16504875e0_f64) * t6973 + t3933 + F::cast_from(0.11038e0_f64) * t5562 - F::cast_from(0.27595e-1_f64) * t6977 + F::cast_from(0.16557e0_f64) * t6980 - F::cast_from(0.82785e-1_f64) * t6983;
    (t7004, t7019)
}
