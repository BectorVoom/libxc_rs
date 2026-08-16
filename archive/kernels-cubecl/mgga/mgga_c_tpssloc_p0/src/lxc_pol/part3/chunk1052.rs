//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1052/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1052<F: Float>(t10296: F, t10298: F, t10302: F, t13567: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t1540: F, t2394: F) -> (F, F) {
    let t13592 = -t13567 - F::cast_from(0.33218518518518518518e0_f64) * t13569 + F::cast_from(0.11958666666666666667e1_f64) * t13572 - F::cast_from(0.39862222222222222222e0_f64) * t13575 - F::cast_from(0.19931111111111111111e0_f64) * t13578 - F::cast_from(0.17938e1_f64) * t13581 + F::cast_from(0.11958666666666666667e1_f64) * t13584 + F::cast_from(0.59793333333333333334e0_f64) * t13587 - F::cast_from(0.18257037037037037037e0_f64) * t10296 + F::cast_from(0.54771111111111111111e-1_f64) * t10302 + F::cast_from(0.18257037037037037037e-1_f64) * t10298;
    let t13598 = t2394 * t1540;
    (t13592, t13598)
}
