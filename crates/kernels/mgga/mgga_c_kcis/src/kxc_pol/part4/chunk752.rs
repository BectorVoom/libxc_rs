//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 752/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk752<F: Float>(t1023: F, t4999: F, t1020: F, t1714: F, t2822: F, t1662: F, t2855: F, t1021: F, t1774: F, t2825: F, t1092: F, t1773: F, t3182: F, t1769: F, t2861: F, t1767: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5000 = t4999 * t1023;
    let t5001 = t1020 * t5000;
    let t5003 = t2822 * t1714;
    let t5005 = t2855 * t1662;
    let t5006 = t1021 * t5005;
    let t5007 = t1020 * t5006;
    let t5010 = t2825 * t1774;
    let t5011 = t1092 * t5010;
    let t5013 = t3182 * t1773;
    let t5014 = t1021 * t5013;
    let t5015 = t1092 * t5014;
    let t5017 = t2861 * t1769;
    let t5019 = t2855 * t1767;
    (t5000, t5001, t5003, t5005, t5006, t5007, t5010, t5011, t5013, t5014, t5015, t5017, t5019)
}
