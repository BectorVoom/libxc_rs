//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1021/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1021<F: Float>(t10722: F, t8240: F, t11705: F, t6465: F, t2184: F, t24031: F, t3308: F, t1592: F, t24035: F, t11708: F, t6425: F, t10836: F, t7601: F, t2147: F, t2608: F, t38168: F) -> (F, F, F, F, F, F, F) {
    let t39859 = t8240 * t10722;
    let t39863 = t6465 * t11705;
    let t39866 = t2184 * t3308 * t24031;
    let t39869 = t1592 * t3308 * t24035;
    let t39874 = t6425 * t11708;
    let t39879 = t7601 * t10836;
    let t39882 = t2147 * t38168 * t2608;
    (t39859, t39863, t39866, t39869, t39874, t39879, t39882)
}
