//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1196/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1196<F: Float>(t1070: F, t5801: F, t1095: F, t1938: F, t5830: F, t5775: F, t2782: F, t5804: F, t1979: F, t7474: F, t1915: F, t2793: F, t1088: F, t5829: F, t1107: F, t1977: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20896 = t1070 * t5801;
    let t20905 = t1938 * t1095;
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20918 = t2782 * t5804;
    let t20975 = t7474 * t1979;
    let t21087 = t2793 * t1915;
    let t21090 = t1088 * t5829;
    let t21093 = t1977 * t1107;
    (t20896, t20905, t20908, t20911, t20918, t20975, t21087, t21090, t21093)
}
