//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 979/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk979<F: Float>(t783: F, t8633: F, t1022: F, t1835: F, t2925: F, t701: F, t2610: F, t795: F, t8720: F, t313: F, t769: F, t8637: F, t10007: F, t8502: F, t10012: F, t1865: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24390 = t8633 * t783;
    let t24446 = t1022 * t1835;
    let t24451 = t2925 * t701;
    let t24474 = t2610 * t24451;
    let t24478 = t2610 * t24446;
    let t24487 = t795 * t8720;
    let t24488 = t313 * t24487;
    let t24496 = t769 * t8637;
    let t24501 = t10007 * t8502;
    let t24505 = t10012 * t8502;
    let t24536 = t1022 * t1865;
    (t24390, t24474, t24478, t24487, t24488, t24496, t24501, t24505, t24536)
}
