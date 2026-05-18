//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1147/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1147<F: Float>(t1020: F, t1692: F, t568: F, t1769: F, t6985: F, t1041: F, t17095: F, t1753: F, t6864: F, t2593: F, t5367: F, t1024: F, t154: F, t16335: F, t1634: F, t16341: F, t16356: F, t16373: F, t1706: F, t179: F, t19867: F, t19910: F, t19911: F, t19913: F, t19932: F, t2586: F, t2592: F, t5181: F, t5225: F, t581: F, t612: F, t615: F, t616: F, t6929: F) -> (F, F, F, F, F) {
    let t19933 = t1020 * t1692;
    let t19934 = t19933 * t568;
    let t19938 = t1769 * t6985;
    let t19947 = t17095 * t1041;
    let t19949 = t6864 * t1753;
    let t19953 = t2593 * t5367;
    let t19957 = t19910 - F::new(7.0) / F::new(8.0) * t19911 - F::new(7.0) / F::new(16.0) * t19913 + F::new(3.0) / F::new(16.0) * t1706 * t581 * t6929 * t568 + F::new(3.0) / F::new(16.0) * t1706 * t581 * t2586 * t1692 - F::new(3.0) / F::new(4.0) * t5225 * t581 * t2586 * t1634 + F::new(5.0) / F::new(4.0) * t16373 * t581 * t1024 * t5181 - F::new(3.0) / F::new(4.0) * t19932 * t154 * t19934 + F::new(0.12004725073059526352e-1) * t19938 - F::new(0.85748036236139473944e-3) * t612 * t615 * t616 * t19867 + F::new(0.12004725073059526352e0) * t16335 + F::new(0.40015750243531754508e-2) * t16341 - F::new(0.12004725073059526352e-1) * t16356 + F::new(0.15117061203111996147e0) * t19947 + F::new(0.12862205435420921092e-2) * t2592 * t179 * t19949 + F::new(0.42874018118069736972e-3) * t2592 * t179 * t19953;
    (t19933, t19934, t19949, t19953, t19957)
}
