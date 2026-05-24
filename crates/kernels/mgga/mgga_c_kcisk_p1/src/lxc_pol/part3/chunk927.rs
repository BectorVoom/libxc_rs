//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 927/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk927<F: Float>(t13526: F, t13530: F, t13546: F, t13552: F, t13595: F, t13598: F, t13601: F, t13605: F, t13609: F, t13612: F, t13616: F, t13630: F, t13634: F, t13636: F) -> F {
    let t13734 = -F::new(0.66228e0) * t13595 + F::new(0.33114e0) * t13598 - F::new(0.99342e0) * t13601 + F::new(0.11038e0) * t13605 - F::cast_from(0.73586666666666666666e-1_f64) * t13609 - F::new(0.16557e0) * t13612 - F::new(0.5519e0) * t13616 + F::new(0.258925e1) * t13630 - F::cast_from(0.412621875e-1_f64) * t13634 + F::new(0.16504875e0) * t13636 - F::cast_from(0.60384999999999999999e0_f64) * t13546 + F::new(0.181155e1) * t13552 - F::cast_from(0.40256666666666666668e0_f64) * t13526 + F::cast_from(0.20128333333333333333e0_f64) * t13530;
    t13734
}
