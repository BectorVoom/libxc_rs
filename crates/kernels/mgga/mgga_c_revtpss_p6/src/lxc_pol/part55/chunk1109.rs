//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1109/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1109<F: Float>(t2108: F, t34033: F, t34168: F, t34191: F, t34193: F, t34195: F, t34198: F, t34203: F, t34244: F, t34250: F, t34253: F, t34399: F, t7359: F, t8109: F, t8158: F, t8463: F, t8764: F) -> F {
    let t34795 = t2108 * t34399 - F::cast_from(2.0_f64) * t7359 * t8158 + t8109 * t8764 + t34033 - t34168 + t34191 - t34193 - t34195 - t34198 + t34203 + t34244 - t34250 - t34253 - t8463;
    t34795
}
