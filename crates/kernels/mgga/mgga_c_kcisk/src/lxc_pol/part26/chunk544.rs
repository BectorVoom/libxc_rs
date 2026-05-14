//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 544/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk544<F: Float>(t4208: F, t1413: F, t1481: F, t3507: F, t492: F, t497: F, sigma0: F) -> (F, F, F, F, F) {
    let t4209 = t4208 * sigma0;
    let t4214 = t1481 * t1413;
    let t4215 = t4214 * sigma0;
    let t4223 = t3507 * t492;
    let t4229 = t492 * t497;
    (t4209, t4214, t4215, t4223, t4229)
}
