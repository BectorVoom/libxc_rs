//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 924/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk924<F: Float>(t1089: F, t31520: F, t31521: F, t368: F, t151: F, t7731: F, t950: F, t3378: F, t7560: F, t30049: F, t7461: F, t2104: F, t7610: F) -> (F, F, F, F, F) {
    let t31805 = t31520 * t1089 * t368 * t31521;
    let t31806 = F::new(0.64311027177104605458e-3) * t31805;
    let t31811 = t151 * t7731 * t950;
    let t31824 = t3378 * t7560;
    let t31839 = t30049 * t7461;
    let t31840 = F::new(0.42874018118069736972e-3) * t31839;
    let t31849 = t7610 * t2104;
    (t31806, t31811, t31824, t31840, t31849)
}
