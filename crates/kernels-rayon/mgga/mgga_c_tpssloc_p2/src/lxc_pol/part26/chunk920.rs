//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 920/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk920(t2897: f64, t942: f64, t2929: f64, t938: f64, t10523: f64, t315: f64, t10524: f64, t2932: f64, t10544: f64, t10530: f64, t10538: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64) -> (f64, f64, f64, f64, f64) {
    let t10820 = t2897 * t942;
    let t10825 = t938 * t2929;
    let t10828 = t315 * t10523;
    let t10829 = t10524 * t2932;
    let t10832 = 0.53272592592592592592e-1_f64 * t10544;
    let t10843 = -t10832 - 0.2283111111111111111e-1_f64 * t10556 + 0.11415555555555555555e-1_f64 * t10558 - 0.34246666666666666665e-1_f64 * t10560 + 0.17123333333333333333e-1_f64 * t10562 - 0.19025925925925925925e-1_f64 * t10566 + 0.68493333333333333331e-1_f64 * t10569 - 0.34246666666666666665e-1_f64 * t10530 - 0.10274e0_f64 * t10572 + 0.10274e0_f64 * t10538 - 0.17123333333333333333e-1_f64 * t10575;
    (t10820, t10825, t10828, t10829, t10843)
}
