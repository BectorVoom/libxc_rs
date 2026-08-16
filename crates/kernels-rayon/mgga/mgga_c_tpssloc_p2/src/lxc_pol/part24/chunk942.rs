//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 942/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk942(t10633: f64, t959: f64, t10544: f64, t10530: f64, t10538: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64) -> (f64, f64) {
    let t10635 = 0.10254018858216406658e4_f64 * t959 * t10633;
    let t10636 = 0.55403703703703703703e-1_f64 * t10544;
    let t10647 = -t10636 - 0.23744444444444444444e-1_f64 * t10556 + 0.11872222222222222222e-1_f64 * t10558 - 0.35616666666666666666e-1_f64 * t10560 + 0.17808333333333333333e-1_f64 * t10562 - 0.19787037037037037037e-1_f64 * t10566 + 0.71233333333333333332e-1_f64 * t10569 - 0.35616666666666666666e-1_f64 * t10530 - 0.10685e0_f64 * t10572 + 0.10685e0_f64 * t10538 - 0.17808333333333333333e-1_f64 * t10575;
    (t10635, t10647)
}
