//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 758/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk758<F: Float>(t35348: F, t35545: F, t258: F, t33253: F, t6752: F, t193: F, t1424: F, t6940: F, t729: F, t762: F, t1091: F, t33771: F, t10079: F, t33715: F, t2599: F, t33759: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t35546 = t35348 + t35545;
    let t35547 = t35546 * t258;
    let t35549 = t33253 * t6752;
    let t35550 = t193 * t35549;
    let t35553 = t1424 * t6940;
    let t35555 = t729 * t762 * t35553;
    let t35558 = t33771 * t1091;
    let t35559 = t10079 * t35558;
    let t35562 = t33715 * t1091;
    let t35563 = t2599 * t35562;
    let t35566 = t33759 * t1091;
    (t35546, t35547, t35549, t35550, t35553, t35555, t35558, t35559, t35562, t35563, t35566)
}
