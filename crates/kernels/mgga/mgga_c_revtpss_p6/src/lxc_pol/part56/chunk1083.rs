//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1083/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1083<F: Float>(t33572: F, t571: F, t4245: F, t8453: F, t508: F, t1310: F, t33639: F, t1843: F, t32171: F, t5517: F, t8454: F, t4241: F, t8441: F, t8621: F) -> (F, F, F, F, F, F, F) {
    let t125184 = t571 * t33572;
    let t125209 = t4245 * t8453;
    let t125211 = F::cast_from(2.0_f64) * t125209 * t508;
    let t125213 = F::cast_from(2.0_f64) * t33639 * t1310;
    let t125215 = F::cast_from(2.0_f64) * t32171 * t1843;
    let t125217 = F::cast_from(2.0_f64) * t8454 * t5517;
    let t125228 = t8621 * t8441 * t4241;
    (t125184, t125209, t125211, t125213, t125215, t125217, t125228)
}
