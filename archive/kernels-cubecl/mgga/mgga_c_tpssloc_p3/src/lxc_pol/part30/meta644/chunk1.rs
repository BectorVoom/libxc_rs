//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2056/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056<F: Float>(t23437: F, t4630: F, t25641: F, t82943: F, t1933: F, t1937: F, t3966: F, t25655: F, t82895: F, t25661: F, t1036: F, t25664: F) -> (F, F, F, F, F, F) {
    let t88548 = t23437 * t4630 / F::cast_from(216.0_f64);
    let t88566 = F::cast_from(0.16149102437656156342e-2_f64) * t82943 * t25641;
    let t88569 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t3966 * t1937;
    let t88575 = F::cast_from(0.40372756094140390856e-3_f64) * t82895 * t25655;
    let t88577 = F::cast_from(0.20186378047070195428e-3_f64) * t82895 * t25661;
    let t88582 = t25664 * t1036 / F::cast_from(1152.0_f64);
    (t88548, t88566, t88569, t88575, t88577, t88582)
}
