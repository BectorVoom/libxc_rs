//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 945/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk945<F: Float>(t1175: F, t5684: F, t2083: F, t3587: F, t2188: F, t3598: F, t1173: F, t5700: F, t19144: F, t19102: F, t13009: F, t3559: F, t5690: F, t12948: F, t13263: F, t19106: F, t19111: F, t19121: F, t19134: F, t19138: F, t19142: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19170 = t5684 * t1175;
    let t19173 = t2083 * t3587;
    let t19182 = t3598 * t2188;
    let t19185 = t1173 * t5700;
    let t19192 = t1173 * t19144;
    let t19199 = 0.18344444444444444444e-2 * t19102;
    let t19206 = t13009 * t2083;
    let t19207 = t19206 * t3559;
    let t19211 = t3598 * t5684;
    let t19212 = t19211 * t1175;
    let t19214 = t5690 * t3587;
    let t19217 = -0.45861111111111111112e-2 * t19111 - 0.11006666666666666667e-1 * t19121 + 0.8255e-2 * t19138 + 0.3302e-1 * t19134 + 0.14865e-1 * t19207 - t13263 + 0.30268333333333333334e-1 * t19106 - 0.8255e-2 * t19142 - 0.1982e-1 * t19212 - 0.991e-2 * t19214 - 0.27516666666666666666e-2 * t12948;
    (t19170, t19173, t19182, t19185, t19192, t19199, t19207, t19212, t19214, t19217)
}
