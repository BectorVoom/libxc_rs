//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1405/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1405<F: Float>(t10160: F, t1600: F, t113: F, t9950: F, t2147: F, t6086: F, t19877: F, t33352: F, t26278: F, t9243: F, t10164: F, t1584: F, t20838: F, t25662: F, t25665: F, t25667: F, t26251: F, t30004: F, t30009: F, t30033: F, t30038: F, t9184: F) -> (F, F) {
    let t34071 = t1600 * t10160;
    let t34077 = t9950 * t113;
    let t34079 = t2147 * t6086 * t34077;
    let t34082 = t19877 * t6086 * t33352;
    let t34084 = t26278 * t9243;
    let t34089 = 0.29272321618148349056e-1 * t30004 - 0.34930954652346593433e-1 * t30009 + 0.12459097221822660494e0 * t25662 + 0.64025200389650807209e-1 * t34071 - 0.15602799132097683414e1 * t26251 * t9184 + t25665 - t25667 - 0.43341108700271342816e-1 * t1584 * t10164 - 0.17465477326173296717e-1 * t34079 - 0.20958572791407956061e0 * t34082 + 0.34930954652346593433e-1 * t34084 + 0.14457274399185490173e-4 * t20838 + 0.87816964854445047168e-1 * t30033 - 0.16463622957338778997e-1 * t30038;
    (t34077, t34089)
}
