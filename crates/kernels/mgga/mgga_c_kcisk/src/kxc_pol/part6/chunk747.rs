//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 747/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk747<F: Float>(t15291: F, t213: F, t12476: F, t2957: F, t12485: F, t866: F, t68: F, t71: F, t3: F, t2966: F, t873: F, t80: F) -> (F, F, F, F, F, F, F, F) {
    let t15292 = t15291 * t213;
    let t15294 = t2957 * t12476;
    let t15296 = t866 * t12485;
    let t15298 = t68 * t12485;
    let t15300 = F::cast_from(1.0_f64)/pow_3_2::<F>(t71);
    let t15301 = t15300 * t3;
    let t15302 = t15301 * t213;
    let t15304 = t2966 * t12476;
    let t15306 = t873 * t12485;
    let t15308 = t80 * t12476;
    (t15292, t15294, t15296, t15298, t15302, t15304, t15306, t15308)
}
