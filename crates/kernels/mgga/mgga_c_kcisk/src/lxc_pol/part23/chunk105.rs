//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 105/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk105<F: Float>(t303: F, t306: F, t309: F, t315: F) -> (F, F, F) {
    let t330 = 0.51785e1 * t306 + 0.905775e0 * t303 + 0.1100325e0 * t309 + 0.1241775e0 * t315;
    let t333 = 1.0 + 0.29608574643216675549e2 / t330;
    let t334 = f64::ln(t333);
    (t330, t333, t334)
}
