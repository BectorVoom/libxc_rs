//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 862/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk862<F: Float>(t10443: F, t5409: F, t1882: F, t5327: F, t5311: F, t15402: F, t18514: F, t4139: F, t15386: F, t15385: F, t15195: F, t4261: F, t4266: F, t1240: F, t2766: F, t4141: F) -> (F, F, F, F, F, F, F, F) {
    let t19479 = t10443 * t5409;
    let t19482 = t1882 * t5327;
    let t19484 = t1882 * t5311;
    let t19486 = t15402 * t18514;
    let t19487 = t4139 * t19486;
    let t19490 = t15386 * t18514;
    let t19491 = t15385 * t19490;
    let t19494 = t15195 * t4261;
    let t19497 = t15195 * t4266;
    let t19500 = t2766 * t1240;
    let t19501 = t19500 * t4141;
    (t19479, t19482, t19484, t19487, t19491, t19494, t19497, t19501)
}
