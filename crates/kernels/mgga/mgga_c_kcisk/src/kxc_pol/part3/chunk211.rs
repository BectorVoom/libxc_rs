//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 211/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk211<F: Float>(t879: F, t880: F, t45: F, t68: F, t93: F, t98: F, t867: F, t869: F, t874: F, t877: F) -> (F, F, F, F, F, F) {
    let t881 = t879 * t880;
    let t884 = t45 * t68;
    let t889 = t45 * t93;
    let t890 = t98 * t98;
    let t891 = F::new(1.0) / t890;
    let t896 = -F::cast_from(0.86308333333333333334e0_f64) * t867 - F::new(0.301925e0) * t869 - F::new(0.5501625e-1) * t874 - F::new(0.82785e-1) * t877;
    (t881, t884, t889, t890, t891, t896)
}
