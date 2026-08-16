//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 973/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk973<F: Float>(t30543: F, t8515: F, t30398: F, t30416: F, t10146: F, t420: F, t576: F, t1083: F, t137: F, t30444: F, t1511: F, t2020: F) -> (F, F, F, F, F, F, F) {
    let t34361 = t30543 * t8515;
    let t34364 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t30398;
    let t34366 = F::cast_from(0.25158473831683321654e-2_f64) * t30416;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34373 = F::cast_from(0.15724046144802076034e-2_f64) * t30444;
    let t34382 = t2020 * t1511;
    (t34361, t34364, t34366, t34368, t34369, t34373, t34382)
}
