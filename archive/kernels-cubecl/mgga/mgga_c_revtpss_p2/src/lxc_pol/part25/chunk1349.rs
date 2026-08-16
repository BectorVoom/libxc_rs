//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1349/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1349<F: Float>(t18163: F, t7003: F, t25861: F, t4254: F, t25188: F, t7316: F, t10426: F, t196: F, t197: F, t2035: F, t28167: F, t8996: F, t9984: F) -> (F, F, F, F, F) {
    let t95013 = F::cast_from(6.0_f64) * t18163 * t7003;
    let t95015 = F::cast_from(12.0_f64) * t4254 * t25861;
    let t95017 = F::cast_from(3.0_f64) * t25188 * t7316;
    let t95019 = t10426 * t196 * t197;
    let t95020 = t95019 * t2035;
    let t95023 = F::cast_from(18.0_f64) * t28167 * t8996 * t9984;
    (t95013, t95015, t95017, t95020, t95023)
}
