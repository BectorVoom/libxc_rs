//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 529/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk529<F: Float>(t2386: F, t45: F, t2394: F, t4761: F, t2063: F, t696: F, t5136: F, t2494: F, t960: F, t2497: F, t965: F, t2502: F, t970: F) -> (F, F, F, F, F, F, F) {
    let t6851 = t45 * t2386;
    let t6856 = t4761 * t2394;
    let t6903 = t696 * t2063;
    let t6906 = t5136 * t2063;
    let t6922 = t960 * t2494;
    let t6924 = t965 * t2497;
    let t6926 = t970 * t2502;
    (t6851, t6856, t6903, t6906, t6922, t6924, t6926)
}
