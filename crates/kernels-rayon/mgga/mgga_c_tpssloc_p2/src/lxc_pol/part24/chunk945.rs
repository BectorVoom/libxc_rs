//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 945/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk945(t10311: f64, t10318: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10589: f64, t10591: f64, t10597: f64, t10600: f64) -> f64 {
    let t10695 = 0.16431333333333333333e0_f64 * t10311 - 0.49293999999999999999e0_f64 * t10318 - 0.39862222222222222223e0_f64 * t10556 + 0.19931111111111111111e0_f64 * t10558 - 0.59793333333333333333e0_f64 * t10560 + 0.29896666666666666667e0_f64 * t10562 - 0.33218518518518518518e0_f64 * t10566 + 0.11958666666666666667e1_f64 * t10569 - 0.17938e1_f64 * t10572 - 0.29896666666666666667e0_f64 * t10575 + 0.1898925e1_f64 * t10589 + 0.3071625e0_f64 * t10591 + 0.142419375e1_f64 * t10597 - 0.76790625e-1_f64 * t10600;
    t10695
}
