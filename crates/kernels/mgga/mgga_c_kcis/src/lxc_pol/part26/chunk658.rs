//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 658/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk658<F: Float>(t4456: F, t7429: F, t286: F, t4318: F, t5469: F, t6939: F, t6942: F, t6946: F, t2079: F, t1572: F, t4338: F, t4345: F, t5562: F, t6958: F, t6965: F, t6971: F, t6973: F, t6977: F, t6980: F, t6983: F) -> (F, F, F, F, F, F) {
    let t7430 = t4456 * t7429;
    let t7431 = t286 * t7430;
    let t7438 = t4318 + F::cast_from(0.11415555555555555555e-1_f64) * t5469 - F::cast_from(0.11415555555555555555e-1_f64) * t6939 + F::cast_from(0.34246666666666666666e-1_f64) * t6942 - F::cast_from(0.17123333333333333333e-1_f64) * t6946;
    let t7443 = t2079 * t2079;
    let t7444 = t7443 * t1572;
    let t7459 = -F::new(0.17648625e1) * t6958 + F::new(0.3529725e1) * t6965 + t4338 + F::cast_from(0.34431666666666666666e0_f64) * t5469 - F::cast_from(0.34431666666666666667e0_f64) * t6939 + F::new(0.103295e1) * t6942 - F::new(0.516475e0) * t6946 + F::new(0.31558125e0) * t6971 + F::new(0.6311625e0) * t6973 + t4345 + F::cast_from(0.13892666666666666667e0_f64) * t5562 - F::cast_from(0.34731666666666666667e-1_f64) * t6977 + F::new(0.20839e0) * t6980 - F::new(0.104195e0) * t6983;
    (t7430, t7431, t7438, t7443, t7444, t7459)
}
