//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1324/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1324(t2514: f64, t2492: f64, t2548: f64, t2490: f64, t2595: f64, t39490: f64, t39492: f64, t39495: f64, t39498: f64, t39501: f64, t39506: f64, t39508: f64, t39510: f64, t39512: f64, t39515: f64) -> (f64, f64, f64, f64, f64) {
    let t39871 = t2514 * t2514;
    let t39875 = t2492 * t2492;
    let t39886 = t2548 * t2548;
    let t39894 = 1.0_f64 / t2490 / t2595;
    let t39909 = -0.28769444444444444444e1_f64 * t39490 + 0.27618666666666666667e2_f64 * t39492 - 0.10229135802469135803e2_f64 * t39495 + 0.89504938271604938273e1_f64 * t39498 + 0.31310740740740740741e1_f64 * t39501 + 0.366775e-1_f64 * t39506 - 0.58684e0_f64 * t39508 + 0.65204444444444444445e0_f64 * t39510 + 0.5705388888888888889e0_f64 * t39512 + 0.13490888888888888889e1_f64 * t39515;
    (t39871, t39875, t39886, t39894, t39909)
}
