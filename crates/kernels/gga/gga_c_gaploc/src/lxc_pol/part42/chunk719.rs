//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 719/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk719<F: Float>(t20368: F, t44386: F, t1358: F, t23915: F, t161: F, t37573: F, t2339: F, t13396: F, t2299: F, t488: F, t42640: F, t42644: F, t42647: F, t42651: F, t2321: F, t38019: F, t9074: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44387 = t20368 * t44386;
    let t44390 = 0.18970004423784099732e-1 * t1358 * t23915 * t44387;
    let t44391 = t37573 * t161;
    let t44394 = 0.94850022118920498663e-2 * t1358 * t44391 * t2339;
    let t44403 = 0.31616674039640166221e-2 * t1358 * t2299 * t13396 * t488;
    let t44409 = 0.142275033178380748e-1 * t42640;
    let t44410 = 0.33197507741622174533e-1 * t42644;
    let t44411 = 0.56910013271352299199e-1 * t42647;
    let t44413 = 0.28455006635676149599e-1 * t42651;
    let t44415 = t9074 * t38019 * t2321;
    (t44387, t44390, t44394, t44403, t44409, t44410, t44411, t44413, t44415)
}
