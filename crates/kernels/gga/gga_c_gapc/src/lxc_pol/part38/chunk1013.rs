//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1013/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1013<F: Float>(t3708: F, t8863: F, t3714: F, t11450: F, t11451: F, t21115: F, t11512: F, t1736: F, t1743: F, t1749: F, t1: F, t26662: F, t5462: F, t8681: F, t11332: F, t1643: F, t4995: F) -> (F, F, F, F, F, F, F) {
    let t34409 = t8863 * t3708;
    let t34410 = t34409 * t3714;
    let t34413 = t11450 * t11451 * t21115;
    let t34417 = t1743 * t11512 * t1736 * t1749;
    let t34419 = t26662 * t1;
    let t34421 = t5462 * t34419 * t8681;
    let t34424 = t1643 * t4995 * t11332;
    (t34409, t34410, t34413, t34417, t34419, t34421, t34424)
}
