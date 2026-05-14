//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 977/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk977<F: Float>(t3216: F, t788: F, t2201: F, t785: F, t3190: F, t481: F, t551: F, t552: F, t113: F, t8820: F, t2148: F, t2147: F, t1632: F, t549: F, t2670: F, t2731: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9434 = t788 * t3216;
    let t9436 = t2201 * t785 * t9434;
    let t9439 = t3190 * t481;
    let t9441 = t551 * t552 * t9439;
    let t9445 = t8820 * t113;
    let t9446 = t2148 * t9445;
    let t9447 = t2147 * t9446;
    let t9451 = t1632 * t3216;
    let t9452 = t551 * t9451;
    let t9453 = t549 * t9452;
    let t9458 = t2670 * t2731;
    (t9434, t9436, t9439, t9441, t9445, t9446, t9447, t9452, t9453, t9458)
}
