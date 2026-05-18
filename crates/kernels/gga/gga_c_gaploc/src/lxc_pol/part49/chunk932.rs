//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 932/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk932<F: Float>(t10340: F, t1445: F, t1562: F, t2293: F, t12919: F, t4953: F, t3116: F, t8097: F, t10215: F, t1429: F, t2365: F, t2366: F) -> (F, F, F, F) {
    let t42015 = t1562 * t1445 * t10340 * t2293;
    let t42018 = F::new(0.69017266717057349418e1) * t4953 * t12919;
    let t42022 = F::new(0.69017266717057349418e1) * t1562 * t1445 * t8097 * t3116;
    let t42026 = t1429 * t2365 * t2366 * t10215;
    (t42015, t42018, t42022, t42026)
}
