//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 536/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk536<F: Float>(t1200: F, t1809: F, t388: F, t4772: F, t387: F, t1187: F, t1801: F, t3474: F, t3438: F, t4984: F, t3437: F, t284: F, t374: F, t3217: F, t41: F) -> (F, F, F, F, F, F, F, F) {
    let t5065 = t1809 * t1200;
    let t5067 = t388 * t4772;
    let t5068 = t387 * t5067;
    let t5069 = t1187 * t5068;
    let t5071 = t3474 * t1801;
    let t5073 = t3438 * t4984;
    let t5074 = t3437 * t5073;
    let t5076 = t374 * t284;
    let t5077 = t41 * t3217;
    (t5065, t5068, t5069, t5071, t5073, t5074, t5076, t5077)
}
