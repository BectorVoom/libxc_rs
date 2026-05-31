//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 838/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk838<F: Float>(t10995: F, t414: F, t1258: F, t3490: F, t3504: F, t25: F, t3533: F, t1251: F, t1259: F, t2888: F, t3501: F, t3500: F, t3521: F) -> (F, F, F, F, F, F, F) {
    let t10996 = t414 * t10995;
    let t10999 = t1258 * t1258;
    let t11000 = F::cast_from(1.0_f64) / t10999;
    let t11009 = t3490 * t3504;
    let t11013 = t25 * t3533;
    let t11014 = t1251 * t11013;
    let t11020 = t2888 * t1259;
    let t11034 = t3490 * t3501;
    let t11041 = t3500 * t3521;
    (t10996, t11000, t11009, t11014, t11020, t11034, t11041)
}
