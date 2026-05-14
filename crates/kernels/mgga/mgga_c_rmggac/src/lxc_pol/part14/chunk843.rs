//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 843/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk843<F: Float>(t3928: F, t5199: F, t645: F, t118: F, t1986: F, t352: F, t39866: F, t7717: F, t1971: F, t2144: F, t7230: F, t8834: F, t2318: F, t326: F, t333: F, t236: F, t321: F, t7248: F, t8666: F) -> (F, F, F, F, F) {
    let t40307 = t3928 * t645 * t5199;
    let t40313 = t1986 * t118 * t39866 * t352;
    let t40314 = t7717 * t40313;
    let t40319 = t7230 * t1971 * t2144 * t8834 * t352;
    let t40323 = t1986 * t326 * t2318 * t333;
    let t40324 = t7717 * t40323;
    let t40329 = t7230 * t7248 * t236 * t8666 * t321;
    (t40307, t40314, t40319, t40324, t40329)
}
