//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2070/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2070<F: Float>(t3813: F, t651: F, t7741: F, t18153: F, t1936: F, t18163: F, t7742: F, t28063: F, t4254: F, t1937: F, t75485: F, t18227: F, t6993: F) -> (F, F, F, F, F, F) {
    let t97617 = F::new(2.0) * t651 * t3813 * t7741;
    let t97629 = F::new(2.0) * t651 * t18153 * t1936;
    let t97639 = F::new(2.0) * t18163 * t7742;
    let t97641 = F::new(4.0) * t4254 * t28063;
    let t97643 = F::new(2.0) * t75485 * t1937;
    let t97645 = F::new(4.0) * t18227 * t6993;
    (t97617, t97629, t97639, t97641, t97643, t97645)
}
