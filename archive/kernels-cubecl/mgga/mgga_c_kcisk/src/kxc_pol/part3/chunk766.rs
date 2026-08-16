//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 766/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk766<F: Float>(t11774: F, t11228: F, t719: F, t735: F, t10522: F, t641: F, t746: F, t741: F, t5310: F, t5327: F, t10431: F, t5322: F, sigma2: F) -> (F, F, F, F) {
    let t11775 = t11774 * sigma2;
    let t11776 = t719 * t11228;
    let t11777 = t735 * t11776;
    let t11778 = t11775 * t11777;
    let t11780 = t641 * t10522;
    let t11781 = t746 * t11780;
    let t11782 = t741 * t11781;
    let t11784 = t5310 * t5327;
    let t11786 = t5322 * t10431;
    (t11778, t11782, t11784, t11786)
}
