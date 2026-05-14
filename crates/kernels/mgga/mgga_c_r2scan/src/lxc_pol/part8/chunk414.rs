//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 414/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk414<F: Float>(t1512: F, t41: F, t410: F, t425: F, t2: F, t424: F, t464: F, t1381: F, t1497: F, t453: F) -> (F, F, F, F, F) {
    let t1513 = t41 * t1512;
    let t1515 = t410 * t425;
    let t1520 = t424 * t2;
    let t1521 = t1520 * t464;
    let t1524 = t1497 * t1381 * t453;
    (t1513, t1515, t1520, t1521, t1524)
}
