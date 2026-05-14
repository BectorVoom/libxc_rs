//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 750/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk750<F: Float>(t1445: F, t1562: F, t3116: F, t8097: F, t10215: F, t1429: F, t2365: F, t2366: F, t3153: F, t8072: F, t10497: F, t2437: F, t2441: F, t34131: F, t895: F, t41838: F, t493: F) -> (F, F, F, F, F, F, F) {
    let t42022 = 0.69017266717057349418e1 * t1562 * t1445 * t8097 * t3116;
    let t42026 = t1429 * t2365 * t2366 * t10215;
    let t42029 = 0.35750489951850426669e0 * t3153 * t8072;
    let t42030 = t2437 * t10497;
    let t42032 = t2441 * t10497;
    let t42034 = t895 * t34131;
    let t42036 = t493 * t41838;
    (t42022, t42026, t42029, t42030, t42032, t42034, t42036)
}
