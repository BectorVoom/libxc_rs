//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 926/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk926<F: Float>(t1986: F, t5251: F, t675: F, t2310: F, t35277: F, t1525: F, t236: F, t321: F, t3352: F, t7230: F, t615: F, t833: F) -> (F, F, F, F) {
    let t40365 = t675 * t1986 * t5251;
    let t40367 = t35277 * t2310;
    let t40372 = t7230 * t3352 * t236 * t1525 * t321;
    let t40377 = t7230 * t3352 * t236 * t615 * t833;
    (t40365, t40367, t40372, t40377)
}
