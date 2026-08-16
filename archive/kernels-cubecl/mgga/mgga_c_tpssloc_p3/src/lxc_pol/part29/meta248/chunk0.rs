//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1163/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1163<F: Float>(t25: F, t868: F, t1877: F, t1915: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t221: F, t60: F, t3: F, t607: F) -> (F, F, F, F) {
    let t6671 = t25 * t868;
    let t6678 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t6542 + t1877 * t6666 * t25 / F::cast_from(2.0_f64) - t1877 * t6670 * t6671 / F::cast_from(2.0_f64) + t1877 * t1915 * t606 / F::cast_from(2.0_f64);
    let t6686 = t221 * t60;
    let t6729 = t3 * t607;
    (t6671, t6678, t6686, t6729)
}
