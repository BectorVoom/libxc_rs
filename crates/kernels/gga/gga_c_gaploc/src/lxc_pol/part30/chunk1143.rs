//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1143/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1143<F: Float>(t10012: F, t10627: F, t15482: F, t22633: F, t11053: F, t7419: F, t9805: F, t1835: F, t7572: F, t7573: F, t10914: F, t10915: F, t32897: F, t2615: F, t28406: F, t28408: F, t28410: F, t28415: F, t28419: F, t28421: F, t28423: F, t28425: F, t28427: F, t28437: F, t28441: F, t326: F, t32796: F) -> (F, F) {
    let t33148 = t10012 * t10627;
    let t33151 = 0.5680433474654925878e0 * t22633 * t15482 * t33148;
    let t33153 = t9805 * t11053 * t7419;
    let t33154 = 0.51762950037793012063e1 * t33153;
    let t33155 = t10627 * t1835;
    let t33158 = 0.69017266717057349418e1 * t7572 * t7573 * t33155;
    let t33164 = 0.42900587942220512002e1 * t10914 * t10915 * t32897;
    let t33165 = t33151 - t33154 - t28406 - t28408 + t28410 - t28415 + t33158 + 0.92023022289409799224e1 * t2615 * t326 * t32796 - t33164 - t28419 - t28421 - t28423 + t28425 + t28427 + t28437 - t28441;
    (t33155, t33165)
}
