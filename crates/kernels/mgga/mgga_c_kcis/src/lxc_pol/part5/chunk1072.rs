//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1072/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1072<F: Float>(t19960: F, t5175: F, t5068: F, t5172: F, t1166: F, t6701: F, t1817: F, t5169: F, t1175: F, t6704: F, t375: F, t3393: F, t6669: F, t18570: F, t5142: F, t18653: F, t5134: F) -> (F, F, F, F, F, F, F, F) {
    let t19961 = t5175 * t19960;
    let t19963 = t5172 * t5068;
    let t19965 = t1166 * t6701;
    let t19967 = t5169 * t1817;
    let t19969 = t1175 * t6704;
    let t19970 = t375 * t19969;
    let t19972 = t3393 * t6669;
    let t19974 = t5142 * t18570;
    let t19977 = t5134 * t18653;
    (t19961, t19963, t19965, t19967, t19970, t19972, t19974, t19977)
}
