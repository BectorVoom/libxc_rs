//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1251/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1251<F: Float>(t1616: F, t2207: F, t3016: F, t785: F, t1632: F, t551: F, t6528: F, t9124: F, t1592: F, t9212: F, t2184: F, t9098: F, t7937: F, t8792: F, t8044: F, t9521: F) -> (F, F, F, F, F, F) {
    let t28198 = t2207 * t785 * t1616 * t3016;
    let t28202 = t6528 * t551 * t1632 * t9124;
    let t28206 = t1592 * t551 * t1632 * t9212;
    let t28225 = t2184 * t551 * t1632 * t9098;
    let t28240 = t8792 * t7937;
    let t28255 = t9521 * t8044;
    (t28198, t28202, t28206, t28225, t28240, t28255)
}
