//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 982/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk982<F: Float>(t1196: F, t12581: F, t1298: F, t3798: F, t3800: F, t498: F, t12487: F, t12552: F, t12555: F, t1188: F, t3520: F, t1294: F, t3568: F) -> (F, F, F, F, F, F) {
    let t12583 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t12581;
    let t12584 = t3798 * t1298;
    let t12587 = F::new(1.0) / t3800 / t498;
    let t12592 = t12552 * t12487 * t12555;
    let t12594 = F::cast_from(0.10254018858216406658e4_f64) * t1196 * t12592;
    let t12596 = t3520 * t12487 * t1188;
    let t12598 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t12596;
    let t12599 = t3568 * t1294;
    (t12583, t12584, t12587, t12594, t12598, t12599)
}
