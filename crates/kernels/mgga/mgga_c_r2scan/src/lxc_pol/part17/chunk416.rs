//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 416/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk416<F: Float>(t166: F, t2055: F, t2056: F, t58: F, t758: F, t423: F, t597: F, t761: F, t776: F, t780: F, t1267: F, t261: F, t277: F, t254: F, t39: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2059 = 0.571528e-1 * t2055 * t166 * t2056;
    let t2060 = t758 * t58;
    let t2061 = t2060 * t423;
    let t2062 = t597 * t761;
    let t2063 = t2061 * t2062;
    let t2083 = t776 * t780;
    let t2086 = t261 * t1267 * t277;
    let t2088 = 0.42377972951376424087e0 * t254 * t2086;
    let t2090 = 1.0 / t9 / t39;
    (t2059, t2060, t2061, t2062, t2063, t2083, t2086, t2088, t2090)
}
