//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 819/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk819<F: Float>(t2107: F, t8717: F, t2014: F, t2042: F, t2113: F, t1936: F, t7553: F, t572: F, t2121: F, t8435: F) -> (F, F, F, F, F, F) {
    let t8718 = t2107 * t8717;
    let t8719 = t2014 * t8718;
    let t8728 = F::new(3.0) * t2113 * t2042;
    let t8731 = t7553 * t1936;
    let t8733 = F::new(6.0) * t572 * t8731;
    let t8736 = t8435 * t2121;
    (t8718, t8719, t8728, t8731, t8733, t8736)
}
