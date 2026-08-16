//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1055/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1055(t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10577: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13600: f64, t13601: f64, t13603: f64, t13613: f64) -> f64 {
    let t13615 = -t10577 - 8.0_f64 / 27.0_f64 * t10556 + 2.0_f64 / 27.0_f64 * t10558 - 2.0_f64 / 9.0_f64 * t10560 + t10562 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t13598 + t13600 - t13601 + t13603 - 10.0_f64 / 27.0_f64 * t13569 + 4.0_f64 / 3.0_f64 * t13572 - 4.0_f64 / 9.0_f64 * t13575 - 2.0_f64 / 9.0_f64 * t13578 - 2.0_f64 * t13581 + 4.0_f64 / 3.0_f64 * t13584 + 2.0_f64 / 3.0_f64 * t13587 - t13613 / 3.0_f64;
    t13615
}
