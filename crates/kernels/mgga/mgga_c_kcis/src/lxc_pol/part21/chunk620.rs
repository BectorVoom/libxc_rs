//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 620/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk620<F: Float>(t4567: F, t4939: F, t1003: F, t1662: F, t2894: F, t291: F, t993: F) -> (F, F, F, F) {
    let t4940 = t4939 * t4567;
    let t4943 = t1662 * t1003;
    let t4944 = t2894 * t4943;
    let t4947 = t993 * t291;
    (t4940, t4943, t4944, t4947)
}
