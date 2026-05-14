//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 844/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk844<F: Float>(t1478: F, t2399: F, t1466: F, t6262: F, t681: F, t6266: F, t1479: F, t25184: F, t25189: F, t25254: F, t25258: F, t25290: F, t25294: F, t25448: F, t25452: F, t25456: F, t25459: F, t25463: F, t25467: F, t25471: F, t25474: F, t25480: F, t6210: F, t6216: F, t6219: F, t6225: F, t6391: F, t830: F) -> (F, F, F, F, F, F, F) {
    let t25485 = t2399 * t1478;
    let t25487 = 2.0 / 27.0 * t1466 * t25485;
    let t25488 = t681 * t6262;
    let t25489 = t1466 * t25488;
    let t25491 = t681 * t6266;
    let t25492 = t1466 * t25491;
    let t25498 = -2.0 * t25184 - 4.0 * t25294 - t6216 * t25448 / 9.0 - t6216 * t25452 / 18.0 - t6216 * t25456 / 27.0 - t25459 * t6219 / 9.0 + t25463 / 27.0 - t6216 * t25467 / 9.0 + t6216 * t25471 / 9.0 + 2.0 * t25474 + 4.0 * t25189 - 2.0 * t25258 + t25480 * t1479 / 6.0 - 2.0 * t830 * t6391 + t25487 - t25489 / 9.0 - t25492 / 9.0 - 2.0 / 3.0 * t6210 * t6225 - 2.0 * t25290 - 4.0 * t25254;
    (t25485, t25487, t25488, t25489, t25491, t25492, t25498)
}
