//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 993/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk993<F: Float>(t29997: F, t7963: F, t9029: F, t524: F, t9033: F, t406: F, t463: F, t944: F, t4241: F, t7942: F, t7884: F, t8396: F) -> (F, F, F, F, F) {
    let t33672 = F::cast_from(0.17347256376410398924e1_f64) * t7963 * t29997 * t9029;
    let t33673 = t9033 * t524;
    let t33675 = t944 * t463 * t406;
    let t33681 = F::cast_from(0.34694512752820797848e1_f64) * t7942 * t33673 * t4241;
    let t33682 = t7884 * t8396;
    (t33672, t33673, t33675, t33681, t33682)
}
