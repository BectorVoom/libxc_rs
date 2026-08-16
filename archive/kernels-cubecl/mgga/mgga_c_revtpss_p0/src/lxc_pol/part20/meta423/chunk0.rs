//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1589/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1589<F: Float>(t43880: F, t43907: F, t1132: F, t2439: F, t3418: F, t141: F, t3417: F, t43869: F, t1145: F, t43875: F, t43839: F, t43852: F) -> (F, F, F, F, F, F, F) {
    let t43908 = t43880 + t43907;
    let t43909 = t1132 * t43908;
    let t43911 = t2439 * t3418;
    let t43914 = t141 * t3417 * t43869;
    let t43917 = t141 * t1145 * t43875;
    let t43920 = t141 * t3417 * t43839;
    let t43923 = t141 * t3417 * t43852;
    (t43908, t43909, t43911, t43914, t43917, t43920, t43923)
}
