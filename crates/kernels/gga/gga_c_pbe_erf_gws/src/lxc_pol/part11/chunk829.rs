//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 829/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk829<F: Float>(t18625: F, t31: F, t4: F, t4573: F, t1318: F, t1216: F, t1321: F, t470: F, t1289: F, t1292: F, t13: F, t18515: F, t1276: F, t1285: F, t1291: F, t1274: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18626 = 16.0 * t18625;
    let t18629 = 0.11483710345679012345e-1 * t4 * t4573 * t31;
    let t18637 = t1318 * t1318;
    let t18638 = 1.0 / t18637;
    let t18639 = t1216 * t1216;
    let t18641 = t1321 * t1321;
    let t18642 = 1.0 / t18641;
    let t18645 = 0.91080982599109921211e5 * t470 * t18638 * t18639 * t18642;
    let t18648 = t1289 * t1289;
    let t18651 = t1292 * t1292;
    let t18655 = 0.24954977986735470917e5 * t13 / t18648 * t18515 / t18651;
    let t18658 = 36.0 * t1291 * t1276 * t1285;
    let t18664 = t1285 * t1285;
    let t18667 = 6.0 * t1274 * t18664 * t404;
    (t18626, t18629, t18638, t18639, t18642, t18645, t18655, t18658, t18664, t18667)
}
