//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 855/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk855(t6883: f64, t8480: f64, t2006: f64, t552: f64, t794: f64, t8479: f64, t6897: f64, t8537: f64, t6562: f64, t2053: f64, t2717: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31192 = 0.38381794893125283518e-1_f64 * t6883 * t8480;
    let t31193 = t552 * t2006;
    let t31198 = t794 * t8479;
    let t31200 = 0.82246703342411321825e-2_f64 * t6897 * t31198;
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = 0.41123351671205660912e-2_f64 * t31320;
    let t31332 = t2717 * t2053;
    let t31337 = t857 * t2053;
    (t31192, t31193, t31198, t31200, t31319, t31321, t31332, t31337)
}
