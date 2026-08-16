//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 264/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk264<F: Float>(t344: F, t883: F, t221: F, t967: F, t339: F, t976: F, t191: F) -> (F, F, F, F, F, F) {
    let t978 = t344 * t883;
    let t995 = t221 * t967;
    let t997 = t339 * t995 / F::cast_from(288.0_f64);
    let t998 = t976 * t883;
    let t1008 = t191 * t191;
    let t1009 = F::cast_from(1.0_f64) / t1008;
    (t978, t995, t997, t998, t1008, t1009)
}
