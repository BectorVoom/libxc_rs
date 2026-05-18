//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 338/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk338<F: Float>(t1462: F, t1464: F, t101: F, t492: F, t472: F, t643: F, t8: F, t5: F) -> (F, F, F, F, F) {
    let t1465 = t1462 * t1464;
    let t1468 = t492 * t101;
    let t1469 = t1468 * t472;
    let t1473 = F::new(1.0) / t8 / t643;
    let t1474 = t5 * t1473;
    (t1465, t1468, t1469, t1473, t1474)
}
