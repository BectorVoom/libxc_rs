//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1147/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1147<F: Float>(t2201: F, t2202: F, t6416: F, t133: F, t5052: F, t146: F, t5054: F, t785: F, t788: F, t1604: F, t20829: F, t1603: F, t6257: F, t2158: F, t1550: F, t481: F) -> (F, F, F, F, F, F, F) {
    let t20944 = t2201 * t6416 * t2202;
    let t20946 = t5052 * t133;
    let t20947 = t146 * t20946;
    let t20950 = t20947 * t785 * t788 * t5054;
    let t20952 = t1604 * t20829;
    let t20954 = t6257 * t1603;
    let t20955 = t20954 * t2158;
    let t20957 = t481 * t1550;
    (t20944, t20946, t20950, t20952, t20954, t20955, t20957)
}
