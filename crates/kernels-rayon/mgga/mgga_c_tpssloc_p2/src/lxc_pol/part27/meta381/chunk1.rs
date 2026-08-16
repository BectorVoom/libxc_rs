//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1566/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1566(t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10832: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64, t14409: f64, t14410: f64) -> f64 {
    let t14419 = -t10832 - 0.1522074074074074074e-1_f64 * t10556 + 0.38051851851851851851e-2_f64 * t10558 - 0.11415555555555555555e-1_f64 * t10560 + 0.57077777777777777777e-2_f64 * t10562 - 0.76103703703703703702e-2_f64 * t13598 + 0.76103703703703703701e-2_f64 * t13563 - t14409 + t14410 - 0.19025925925925925925e-1_f64 * t13569 + 0.68493333333333333331e-1_f64 * t13572 - 0.2283111111111111111e-1_f64 * t13575 - 0.11415555555555555555e-1_f64 * t13578 - 0.10274e0_f64 * t13581 + 0.68493333333333333332e-1_f64 * t13584 + 0.34246666666666666666e-1_f64 * t13587 - 0.17123333333333333333e-1_f64 * t13613;
    t14419
}
