//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1288/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1288<F: Float>(t3813: F, t651: F, t7002: F, t18163: F, t7003: F, t25861: F, t4254: F, t25188: F, t7316: F, t10426: F, t196: F, t197: F) -> (F, F, F, F, F) {
    let t95011 = F::new(6.0) * t651 * t3813 * t7002;
    let t95013 = F::new(6.0) * t18163 * t7003;
    let t95015 = F::new(12.0) * t4254 * t25861;
    let t95017 = F::new(3.0) * t25188 * t7316;
    let t95019 = t10426 * t196 * t197;
    (t95011, t95013, t95015, t95017, t95019)
}
