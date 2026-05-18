//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 751/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk751<F: Float>(t10488: F, t1659: F, t1835: F, t1060: F, t1846: F, t3293: F, t696: F, t11578: F, t11580: F, t11583: F, t11586: F, t11588: F, t11590: F, t11593: F, t11596: F, t158: F, t165: F, t173: F) -> F {
    let t11599 = t1659 * t10488;
    let t11602 = t1835 * t10488;
    let t11605 = t1846 * t1060;
    let t11607 = t696 * t3293;
    let t11609 = -F::new(0.4684e-2) * t11578 - F::new(0.3513e-2) * t158 * t11580 + F::new(0.78066666666666666667e-3) * t158 * t11583 - F::new(0.39624999999999999999e-2) * t11586 + F::new(0.26416666666666666666e-2) * t11588 + F::new(0.7925e-3) * t165 * t11590 - F::new(0.17611111111111111111e-3) * t165 * t11593 - F::new(0.7026e-2) * t158 * t11596 + F::new(0.317e-2) * t165 * t11599 + F::new(0.403305e-4) * t173 * t11602 + F::new(0.71734315950379065738e-1) * t11605 - F::new(0.35867157975189532869e-1) * t11607;
    t11609
}
