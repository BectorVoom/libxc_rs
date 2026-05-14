//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1056/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1056<F: Float>(t2376: F, t818: F, t1004: F, t1275: F, t1010: F, t1277: F, t2391: F, t826: F, t1289: F, t1248: F, t35: F, t1256: F, t1216: F, t313: F, t6678: F, t806: F, t810: F, t8316: F, t8323: F, t8326: F, t8329: F, t8337: F, t8344: F, t8347: F, t8350: F) -> (F, F, F, F, F, F, F, F) {
    let t8355 = t2376 * t818;
    let t8358 = t1004 * t1275;
    let t8367 = t1010 * t1277;
    let t8370 = t2391 * t826;
    let t8373 = t1010 * t1289;
    let t8377 = t1248 * t35;
    let t8385 = t1256 * t35;
    let t8395 = 3.0 / 10.0 * t313 * (-10.0 / 27.0 * t8316 + 20.0 / 9.0 * t8377 * t1216 * t806 + 10.0 / 9.0 * t8323 + 5.0 / 3.0 * t8326 - 5.0 * t8329 - 10.0 / 27.0 * t8337 - 20.0 / 9.0 * t8385 * t1216 * t810 + 10.0 / 9.0 * t8344 - 5.0 / 3.0 * t8347 + 5.0 * t8350) - t6678;
    (t8355, t8358, t8367, t8370, t8373, t8377, t8385, t8395)
}
