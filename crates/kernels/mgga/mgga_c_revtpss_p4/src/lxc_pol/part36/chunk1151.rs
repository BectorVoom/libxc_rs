//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1151/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1151<F: Float>(t27884: F, t7286: F, t72: F, t7929: F, t686: F, t7284: F, t7289: F, t27883: F, t786: F, t213: F, t7910: F, t1885: F, t26024: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27885 = t27884 * t7286;
    let t27887 = t7929 * t72;
    let t27888 = t27887 * t686;
    let t27889 = t7284 * t27888;
    let t27891 = t7289 * t27888;
    let t27899 = t786 * t27883;
    let t27900 = t27899 * t7286;
    let t27909 = t213 * t7910;
    let t27921 = t26024 * t1885;
    (t27885, t27887, t27888, t27889, t27891, t27899, t27900, t27909, t27921)
}
