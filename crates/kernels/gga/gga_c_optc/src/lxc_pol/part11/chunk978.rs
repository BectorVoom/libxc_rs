//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 978/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk978<F: Float>(t1392: F, t7947: F, t1434: F, t3902: F, t999: F, t1431: F, t7330: F, t1587: F, t23471: F, t429: F, t1585: F, t1443: F, t3649: F) -> (F, F, F, F, F) {
    let t33492 = t1392 * t7947;
    let t33574 = t999 * t3902 * t1434;
    let t33596 = t1431 * t7330;
    let t33683 = t23471 * t429 * t1587;
    let t33684 = t1585 * t33683;
    let t33724 = t3649 * t1443;
    (t33492, t33574, t33596, t33684, t33724)
}
