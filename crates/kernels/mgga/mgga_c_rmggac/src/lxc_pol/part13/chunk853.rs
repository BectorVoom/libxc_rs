//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 853/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk853<F: Float>(t3352: F, t352: F, t515: F, t7230: F, t8829: F, t1986: F, t2318: F, t305: F, t321: F, t7717: F, t1981: F, t512: F, t676: F, t8512: F) -> (F, F, F) {
    let t39099 = t7230 * t3352 * t515 * t8829 * t352;
    let t39103 = t1986 * t305 * t2318 * t321;
    let t39104 = t7717 * t39103;
    let t39108 = t8512 * t1981 * t676 * t512;
    (t39099, t39104, t39108)
}
