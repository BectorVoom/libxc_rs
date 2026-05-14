//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 705/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk705<F: Float>(t3952: F, t8318: F, t2059: F, t2326: F, t4400: F, t1312: F, t4406: F, t7706: F, t1581: F, t7710: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8319 = t3952 * t8318;
    let t8322 = t2059 * t2326;
    let t8323 = t4400 * t8322;
    let t8324 = t1312 * t8323;
    let t8327 = t4406 * t7706;
    let t8328 = t1312 * t8327;
    let t8331 = t1581 * t7710;
    let t8332 = t1312 * t8331;
    let t8335 = t2326 * t2326;
    (t8319, t8322, t8323, t8324, t8327, t8328, t8331, t8332, t8335)
}
