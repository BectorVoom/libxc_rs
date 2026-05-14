//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 782/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk782<F: Float>(t17836: F, t17837: F, t1614: F, t51: F, t213: F, t1109: F, t679: F, t689: F, t1095: F, t2382: F, t2379: F, t4939: F, t807: F, t236: F, t688: F, t3724: F) -> (F, F, F, F, F, F, F) {
    let t17838 = t17836 * t17837;
    let t17839 = t51 * t1614;
    let t17840 = t17839 * t213;
    let t17841 = t1109 * t679;
    let t17842 = t17841 * t689;
    let t17843 = t17840 * t17842;
    let t17846 = t1095 * t2382;
    let t17847 = t2379 * t17846;
    let t17850 = t4939 * t2382;
    let t17851 = t2379 * t17850;
    let t17854 = t807 * t17850;
    let t17855 = t236 * t688;
    let t17856 = t3724 * t17855;
    (t17838, t17843, t17847, t17850, t17851, t17854, t17856)
}
