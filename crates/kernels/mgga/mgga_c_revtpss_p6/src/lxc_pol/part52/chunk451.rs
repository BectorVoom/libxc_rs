//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 451/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk451<F: Float>(t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t701: F, t682: F) -> F {
    let t2576 = -F::cast_from(0.42198333333333333333e0_f64) * t2502 + F::cast_from(0.84396666666666666666e0_f64) * t2504 + F::cast_from(0.39862222222222222223e0_f64) * t2435 + F::cast_from(0.68258333333333333333e-1_f64) * t2509 + F::cast_from(0.13651666666666666667e0_f64) * t2511 + F::cast_from(0.13692777777777777778e0_f64) * t2439;
    let t2577 = t2576 * t701;
    let t2579 = F::cast_from(1.0_f64) * t682 * t2577;
    t2579
}
