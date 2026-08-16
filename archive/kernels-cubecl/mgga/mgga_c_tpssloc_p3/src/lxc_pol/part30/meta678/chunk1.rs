//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2121/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2121<F: Float>(t109: F, t81438: F, t81440: F, t86589: F, t86591: F, t92121: F, t96713: F, t96716: F, t96719: F, t96721: F, t96724: F, t96726: F, t1268: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t96728 = -t81438 - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t81440 - t92121 - t86589 + t86591 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t96713 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t96716 + t96719 / F::cast_from(2.0_f64) + t96721 / F::cast_from(3.0_f64) + t96724 / F::cast_from(4.0_f64) - t96726 / F::cast_from(8.0_f64);
    let t96729 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t96728);
    let t96731 = F::cast_from(2.0_f64) * t1268 * t96729;
    (t96729, t96731)
}
