//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 811/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk811<F: Float>(t12766: F, t1572: F, t4673: F, t10340: F, t1445: F, t1562: F, t2293: F, t12919: F, t4953: F, t3116: F, t8097: F, t10215: F, t1429: F, t2365: F, t2366: F, t3153: F, t8072: F) -> (F, F, F, F, F, F) {
    let t42008 = 0.47667319935800568892e0 * t1572 * t4673 * t12766;
    let t42015 = t1562 * t1445 * t10340 * t2293;
    let t42018 = 0.69017266717057349418e1 * t4953 * t12919;
    let t42022 = 0.69017266717057349418e1 * t1562 * t1445 * t8097 * t3116;
    let t42026 = t1429 * t2365 * t2366 * t10215;
    let t42029 = 0.35750489951850426669e0 * t3153 * t8072;
    (t42008, t42015, t42018, t42022, t42026, t42029)
}
