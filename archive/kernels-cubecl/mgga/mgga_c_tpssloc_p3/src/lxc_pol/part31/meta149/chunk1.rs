//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 749/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk749<F: Float>(t210: F, t214: F, t4119: F, t2562: F, t2564: F, t2569: F, t2579: F, t2590: F, t4124: F, t4127: F, t4130: F, t4135: F, t787: F) -> (F, F) {
    let t4138 = t210 * t214 * t4119;
    let t4142 = t2562 + F::cast_from(0.38888888888888888888e-2_f64) * t2564 + t2569 + F::cast_from(0.38888888888888888887e-2_f64) * t4124 + F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t4130 + F::cast_from(0.8333333333333333333e-3_f64) * t4135 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t4138 + F::cast_from(0.83333333333333333332e-3_f64) * t2579 - t2590;
    (t4138, t4142)
}
