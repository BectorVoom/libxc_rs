//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1034/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1034(t15291: f64, t213: f64, t12476: f64, t2957: f64, t12485: f64, t866: f64, t68: f64, t71: f64, t3: f64, t2966: f64, t873: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15292 = t15291 * t213;
    let t15294 = t2957 * t12476;
    let t15296 = t866 * t12485;
    let t15298 = t68 * t12485;
    let t15300 = 1.0_f64/pow_3_2(t71);
    let t15301 = t15300 * t3;
    let t15302 = t15301 * t213;
    let t15304 = t2966 * t12476;
    let t15306 = t873 * t12485;
    let t15308 = t80 * t12476;
    (t15292, t15294, t15296, t15298, t15302, t15304, t15306, t15308)
}
