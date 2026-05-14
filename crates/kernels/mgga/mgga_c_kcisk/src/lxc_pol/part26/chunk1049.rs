//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1049/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1049<F: Float>(t529: F, t2110: F, t260: F, t67: F, t21592: F, t26404: F, t26411: F, t6443: F, t1287: F, t1558: F, t2153: F, t26417: F, t27876: F, t382: F, t525: F, t526: F, t6431: F, t6442: F, t6444: F, t8011: F, t8015: F, t8292: F) -> (F,) {
    let t530 = t529 < -0.66725e-1;
    let t27887 = t260 * t67 * t2110;
    let t27893 = t21592 * t26404;
    let t27899 = t6443 * t26411;
    let t27906 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t27876 * t382 - 10.0 / 27.0 * t525 * t8292 * t1287 - 20.0 / 27.0 * t525 * t6431 * t2153 + 80.0 / 81.0 * t27887 * t6444 + 40.0 / 81.0 * t525 * t1558 * t8011 - 280.0 / 243.0 * t6442 * t27893 - 10.0 / 27.0 * t525 * t1558 * t8015 + 40.0 / 81.0 * t6442 * t27899 - 10.0 / 27.0 * t525 * t526 * t26417);
    (t27906,)
}
