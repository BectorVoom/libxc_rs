//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1283/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1283<F: Float>(t2811: F, t6539: F, t1008: F, t26686: F, t13376: F, t1662: F, t4947: F, t14554: F, t4621: F, t4781: F, t27819: F, t6276: F) -> (F, F, F, F) {
    let t101001 = t2811 * t6539;
    let t101003 = t26686 * t101001 * t1008;
    let t101012 = t4947 * t13376 * t1662;
    let t101018 = t14554 * t4781 * t4621;
    let t101028 = t4947 * t27819 * t6276 * t1008;
    (t101003, t101012, t101018, t101028)
}
