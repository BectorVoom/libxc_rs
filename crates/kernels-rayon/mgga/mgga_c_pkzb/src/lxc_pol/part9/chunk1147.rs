//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1147/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1147(t1020: f64, t1692: f64, t568: f64, t1769: f64, t6985: f64, t1041: f64, t17095: f64, t1753: f64, t6864: f64, t2593: f64, t5367: f64, t1024: f64, t154: f64, t16335: f64, t1634: f64, t16341: f64, t16356: f64, t16373: f64, t1706: f64, t179: f64, t19867: f64, t19910: f64, t19911: f64, t19913: f64, t19932: f64, t2586: f64, t2592: f64, t5181: f64, t5225: f64, t581: f64, t612: f64, t615: f64, t616: f64, t6929: f64) -> (f64, f64, f64, f64, f64) {
    let t19933 = t1020 * t1692;
    let t19934 = t19933 * t568;
    let t19938 = t1769 * t6985;
    let t19947 = t17095 * t1041;
    let t19949 = t6864 * t1753;
    let t19953 = t2593 * t5367;
    let t19957 = t19910 - 7.0_f64 / 8.0_f64 * t19911 - 7.0_f64 / 16.0_f64 * t19913 + 3.0_f64 / 16.0_f64 * t1706 * t581 * t6929 * t568 + 3.0_f64 / 16.0_f64 * t1706 * t581 * t2586 * t1692 - 3.0_f64 / 4.0_f64 * t5225 * t581 * t2586 * t1634 + 5.0_f64 / 4.0_f64 * t16373 * t581 * t1024 * t5181 - 3.0_f64 / 4.0_f64 * t19932 * t154 * t19934 + 0.12004725073059526352e-1_f64 * t19938 - 0.85748036236139473944e-3_f64 * t612 * t615 * t616 * t19867 + 0.12004725073059526352e0_f64 * t16335 + 0.40015750243531754508e-2_f64 * t16341 - 0.12004725073059526352e-1_f64 * t16356 + 0.15117061203111996147e0_f64 * t19947 + 0.12862205435420921092e-2_f64 * t2592 * t179 * t19949 + 0.42874018118069736972e-3_f64 * t2592 * t179 * t19953;
    (t19933, t19934, t19949, t19953, t19957)
}
