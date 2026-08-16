//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 886/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk886<F: Float>(t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t5363: F, t5371: F, t577: F, t154: F, t781: F, t202: F, t243: F) -> (F, F, F, F) {
    let t5376 = t1458 * t671;
    let t5381 = F::cast_from(0.45e1_f64) * t5363 * t577 + F::cast_from(0.135e2_f64) * t5371 * t671 + F::cast_from(0.135e2_f64) * t3938 * t1458 + F::cast_from(27.0_f64) * t3941 * t5376 + F::cast_from(0.135e2_f64) * t1401 * t4072;
    let t6546 = t781 * t154;
    let t6589 = F::cast_from(1.0_f64) / t243 / t202;
    (t5376, t5381, t6546, t6589)
}
