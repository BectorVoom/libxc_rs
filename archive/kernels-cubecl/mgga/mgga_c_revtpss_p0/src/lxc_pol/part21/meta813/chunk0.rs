//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2977/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977<F: Float>(t15234: F, t3011: F, t4733: F, t981: F, t15559: F, t3022: F, t15526: F, t15525: F, t2989: F, t52647: F, t52650: F, t52652: F, t52762: F, t52806: F, t52808: F, t52923: F) -> (F, F, F, F, F) {
    let t54238 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t3011 * t15234 * t4733;
    let t54240 = F::cast_from(0.10526802520742363173e2_f64) * t3022 * t15559;
    let t54242 = F::cast_from(0.10389515463408878255e3_f64) * t3022 * t15526;
    let t54245 = F::cast_from(0.10526802520742363173e2_f64) * t981 * t15525 * t2989;
    let t54246 = -t52923 + t52647 + t52650 + t52652 + t52762 - t52806 + t52808 - t54238 - t54240 - t54242 - t54245;
    (t54238, t54240, t54242, t54245, t54246)
}
