//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 931/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk931<F: Float>(t13790: F, t395: F, t1308: F, t4154: F, t1322: F, t3959: F, t3988: F, t6204: F, t1309: F, t1315: F, t13474: F, t13478: F, t13482: F, t13487: F, t13489: F, t13493: F, t13497: F, t13501: F, t13506: F, t13509: F, t3935: F, t3939: F, t405: F, sigma0: F) -> (F, F) {
    let t13791 = t13790 * sigma0;
    let t13792 = t13791 * t395;
    let t13795 = t4154 * t1308;
    let t13799 = t3959 * t1322 * t3988;
    let t13800 = t6204 * t13799;
    let t13803 = F::new(0.10794473229706390328e0) * t3935 * t13474 + F::new(0.10794473229706390328e0) * t3935 * t13478 + F::new(0.28785261945883707542e0) * t13482 * t3939 - F::new(0.35981577432354634425e-1) * t13487 + F::new(0.71963154864709268853e-1) * t3935 * t13489 - F::new(0.10794473229706390328e0) * t13493 * t3939 - F::new(0.53972366148531951639e-1) * t3935 * t13497 - F::new(0.53972366148531951639e-1) * t3935 * t13501 - F::new(0.71963154864709268852e-1) * t3935 * t13506 - F::new(0.10794473229706390328e0) * t3935 * t13509 + F::new(0.5397236614853195164e-1) * t13792 * t405 + F::new(0.53972366148531951639e-1) * t13795 * t1315 + F::new(0.32383419689119170984e0) * t1309 * t13800;
    (t13795, t13803)
}
