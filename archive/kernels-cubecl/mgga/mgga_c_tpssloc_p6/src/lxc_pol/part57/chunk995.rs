//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 995/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk995<F: Float>(t101698: F, t1888: F, t232: F, t6646: F, t112990: F, t112995: F, t121488: F, t121504: F, t121524: F, t121533: F, t126442: F, t126446: F, t126452: F, t126453: F, t1510: F, t812: F) -> F {
    let t127986 = t1888 * t6646 * t101698 * t232;
    let t127990 = -F::cast_from(2.0_f64) * t812 * t121488 * t1510 - F::cast_from(0.82246703342411321824e-2_f64) * t121504 - t126442 + t126446 - F::cast_from(0.16449340668482264365e-1_f64) * t127986 + t126452 + t126453 + F::cast_from(0.82246703342411321824e-2_f64) * t121524 + t112990 + t112995 + F::cast_from(0.76763589786250567036e-1_f64) * t121533;
    t127990
}
