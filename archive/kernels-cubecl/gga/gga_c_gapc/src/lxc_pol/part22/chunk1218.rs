//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1218/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1218<F: Float>(t1: F, t26662: F, t5462: F, t8681: F, t11332: F, t1643: F, t4995: F, t11347: F, t620: F, t1929: F, t3670: F, t11537: F, t3137: F, t505: F, t5059: F, t674: F) -> (F, F, F, F, F, F) {
    let t34419 = t26662 * t1;
    let t34421 = t5462 * t34419 * t8681;
    let t34424 = t1643 * t4995 * t11332;
    let t34426 = t11347 * t620;
    let t34428 = t3670 * t1929;
    let t34433 = t11537 * t3137 * t505 * t674 * t5059;
    (t34419, t34421, t34424, t34426, t34428, t34433)
}
