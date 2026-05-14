//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 735/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk735<F: Float>(t34109: F, t34153: F, t34196: F, t34249: F, t33983: F, t6223: F, t193: F, t1466: F, t301: F, t33808: F, t33994: F, t33998: F, t34003: F, t34008: F, t34013: F, t34015: F, t34017: F, t34019: F, t34022: F, t34025: F, t34054: F, t34058: F, t6210: F, t6216: F, t6219: F, t6225: F, t7581: F, t7614: F, t7684: F, t830: F) -> (F, F, F, F) {
    let t34251 = t34109 + t34153 + t34196 + t34249;
    let t34253 = t33983 * t6223;
    let t34254 = t193 * t34253;
    let t34259 = -t33808 * t6219 / 18.0 + 2.0 * t33994 - t6216 * t33998 / 9.0 - t6216 * t34003 / 18.0 + t6216 * t34008 / 9.0 - t830 * t7684 - 2.0 * t34013 - 4.0 * t34015 + 4.0 * t34017 - 2.0 * t34019 + t1466 * t34022 - 2.0 / 3.0 * t1466 * t34025 - t7581 * t6225 / 3.0 - 2.0 * t34054 - 2.0 / 3.0 * t1466 * t34058 - t301 * t34251 - t1466 * t34254 / 3.0 + t6210 * t7614 / 6.0;
    (t34251, t34253, t34254, t34259)
}
