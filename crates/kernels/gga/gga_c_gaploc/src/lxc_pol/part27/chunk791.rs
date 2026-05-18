//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 791/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk791<F: Float>(t1835: F, t2581: F, t1445: F, t2066: F, t954: F, t7250: F, t7254: F, t2645: F, t4614: F, t2572: F, t4673: F, t1865: F, t2571: F) -> (F, F, F, F, F, F, F) {
    let t7464 = t2581 * t1835;
    let t7465 = t1445 * t7464;
    let t7468 = t2066 * t954;
    let t7473 = t1445 * t7250;
    let t7476 = t1445 * t7254;
    let t7479 = t4614 * t2645;
    let t7482 = t4673 * t2572;
    let t7487 = t2571 * t1865;
    (t7465, t7468, t7473, t7476, t7479, t7482, t7487)
}
