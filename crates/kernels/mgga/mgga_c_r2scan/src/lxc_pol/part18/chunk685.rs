//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 685/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk685<F: Float>(t2099: F, t161: F, t2036: F, t406: F, t1419: F, t726: F, t1416: F, t2035: F, t424: F, t41: F, t236: F, t4715: F, t735: F, t1422: F, t661: F, t230: F, t4911: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5876 = 1.0 / t2099;
    let t5878 = 2184.0 * t161 * t5876;
    let t5883 = t406 * t2036;
    let t5885 = t1419 * t726;
    let t5889 = 60.0 * t1416 * t726;
    let t5890 = t424 * t2035;
    let t5891 = t41 * t5890;
    let t5893 = t4715 * t236;
    let t5895 = 0.16867793133802706421e-1 * t735 * t5893;
    let t5896 = t1422 * t726;
    let t5898 = t1422 * t661;
    let t5901 = 24.0 * t4911 * t230;
    (t5878, t5883, t5885, t5889, t5891, t5895, t5896, t5898, t5901)
}
