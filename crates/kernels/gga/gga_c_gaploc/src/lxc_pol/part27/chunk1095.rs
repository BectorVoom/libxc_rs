//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1095/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1095<F: Float>(t10018: F, t7375: F, t10007: F, t1835: F, t2615: F, t9438: F, t124: F, t15478: F, t3307: F, t813: F, t10013: F, t2464: F, t2684: F) -> (F, F, F, F, F) {
    let t28156 = t7375 * t10018;
    let t28160 = t2615 * t9438 * t10007 * t1835;
    let t28229 = t15478 * t124;
    let t28231 = t813 * t28229 * t3307;
    let t28242 = t2684 * t2464 * t10013;
    (t28156, t28160, t28229, t28231, t28242)
}
