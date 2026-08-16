//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 111/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk111(t11: f64, t1: f64, t344: f64, t21: f64, t86: f64, t345: f64, t347: f64, t30: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t349 = f64::sqrt(t11);
    let t350 = t349 * t1;
    let t351 = t350 * t344;
    let t353 = t21 * t86;
    let t355 = -0.632975e0_f64 * t345 - 0.29896666666666666667e0_f64 * t347 - 0.1023875e0_f64 * t351 - 0.82156666666666666667e-1_f64 * t353;
    let t356 = 1.0_f64 / t30;
    let t357 = t355 * t356;
    let t359 = 1.0_f64 * t340 * t357;
    (t350, t351, t353, t355, t356, t357, t359)
}
