//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 933/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk933(t10159: f64, t898: f64, t2328: f64, t3837: f64, t3160: f64, t8170: f64, t10033: f64, t10153: f64, t10155: f64, t10157: f64, t9751: f64, t9753: f64, t9755: f64, t9758: f64, t9764: f64, t9766: f64, t9768: f64, t9770: f64, t9840: f64, t9842: f64) -> (f64, f64, f64, f64, f64) {
    let t10161 = 0.5848223622634646207e0_f64 * t898 * t10159;
    let t10163 = 0.5848223622634646207e0_f64 * t2328 * t3837;
    let t10164 = t3160 * t8170;
    let t10166 = 0.34631718211362927518e2_f64 * t898 * t10164;
    let t10167 = -t9751 - t9753 + t9755 + t9758 - t9764 + t10033 + t10153 - t10155 + t10157 - t10161 - t10163 + t9766 - t9768 + t9770 + t9840 + t9842 - t10166;
    (t10161, t10163, t10164, t10166, t10167)
}
