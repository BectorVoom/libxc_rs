//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2977/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2977(t15234: f64, t3011: f64, t4733: f64, t981: f64, t15559: f64, t3022: f64, t15526: f64, t15525: f64, t2989: f64, t52647: f64, t52650: f64, t52652: f64, t52762: f64, t52806: f64, t52808: f64, t52923: f64) -> (f64, f64, f64, f64, f64) {
    let t54238 = 0.51947577317044391277e2_f64 * t981 * t3011 * t15234 * t4733;
    let t54240 = 0.10526802520742363173e2_f64 * t3022 * t15559;
    let t54242 = 0.10389515463408878255e3_f64 * t3022 * t15526;
    let t54245 = 0.10526802520742363173e2_f64 * t981 * t15525 * t2989;
    let t54246 = -t52923 + t52647 + t52650 + t52652 + t52762 - t52806 + t52808 - t54238 - t54240 - t54242 - t54245;
    (t54238, t54240, t54242, t54245, t54246)
}
