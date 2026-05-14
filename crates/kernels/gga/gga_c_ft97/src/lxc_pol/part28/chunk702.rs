//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 702/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk702<F: Float>(t32545: F, t492: F, t83: F, t1332: F, t22940: F, t452: F, t5710: F, t5722: F, t110: F, t32077: F, t8411: F, t7165: F, t1871: F, t488: F, t32082: F, t5617: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32546 = t32545 * t492;
    let t32547 = t83 * t32546;
    let t32550 = t22940 * t1332;
    let t32551 = t83 * t32550;
    let t32555 = t452 * t5710 * t5722;
    let t32559 = t8411 * t110 * t32077;
    let t32562 = t7165 * t492;
    let t32564 = t1871 * t488 * t32562;
    let t32568 = t1871 * t110 * t32082;
    let t32571 = t5617 * t1332;
    (t32546, t32547, t32550, t32551, t32555, t32559, t32562, t32564, t32568, t32571)
}
