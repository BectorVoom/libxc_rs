//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 923/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk923<F: Float>(t1986: F, t2318: F, t326: F, t333: F, t7717: F, t236: F, t321: F, t7230: F, t7248: F, t8666: F, t551: F, t7817: F) -> (F, F, F) {
    let t40323 = t1986 * t326 * t2318 * t333;
    let t40324 = t7717 * t40323;
    let t40329 = t7230 * t7248 * t236 * t8666 * t321;
    let t40331 = t7817 * t551;
    (t40324, t40329, t40331)
}
