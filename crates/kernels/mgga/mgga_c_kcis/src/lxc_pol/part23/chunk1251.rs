//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1251/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1251<F: Float>(t1467: F, t1928: F, t1394: F, t4165: F, t28356: F, t4173: F, t27364: F, t5637: F, t27370: F, t28342: F, t4012: F, t8164: F, t94393: F) -> (F, F, F, F, F, F) {
    let t98409 = t1467 * t1928;
    let t98411 = t1394 * t98409 * t4165;
    let t98414 = t1394 * t28356 * t4173;
    let t98417 = t1394 * t27364 * t5637;
    let t98445 = t27370 * t28342 * t4012;
    let t98449 = t1394 * t94393 * t8164;
    (t98409, t98411, t98414, t98417, t98445, t98449)
}
