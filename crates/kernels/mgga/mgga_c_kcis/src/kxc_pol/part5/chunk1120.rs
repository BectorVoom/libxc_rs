//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1120/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1120<F: Float>(t3734: F, t6918: F, t1464: F, t2011: F, t5627: F, t1495: F, t1468: F, t5632: F, t5676: F, t4123: F, t5756: F, t5880: F, t5748: F, t5881: F, t5752: F, t5769: F) -> (F, F, F, F, F, F, F) {
    let t20931 = t3734 * t6918;
    let t20932 = t1464 * t20931;
    let t20934 = t5627 * t2011;
    let t20935 = t1495 * t20934;
    let t20936 = t1468 * t20935;
    let t20937 = t1464 * t20936;
    let t20939 = t5632 * t5676;
    let t20940 = t4123 * t20939;
    let t20941 = t1464 * t20940;
    let t20943 = t5756 * t5880;
    let t20944 = t1468 * t20943;
    let t20945 = t1464 * t20944;
    let t20947 = t5748 * t5881;
    let t20948 = t1464 * t20947;
    let t20950 = t5752 * t5769;
    (t20932, t20934, t20937, t20941, t20945, t20948, t20950)
}
