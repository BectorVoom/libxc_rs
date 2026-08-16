//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 770/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk770<F: Float>(t15959: F, t7824: F, t446: F, t10993: F, t11022: F, t11024: F, t11026: F, t11070: F, t11404: F, t11417: F, t11659: F, t11781: F, t15934: F, t15938: F, t15942: F, t15945: F, t15948: F, t15953: F, t15957: F, t7775: F, t8190: F, t8192: F) -> (F, F) {
    let t15960 = t7824 * t15959;
    let t15961 = t446 * t15960;
    let t15966 = t15934 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15938 + t15942 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15945 - F::cast_from(5.0_f64) / F::cast_from(81.0_f64) * t15948 - t10993 - t11022 - t11024 + t11026 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15953 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t15957 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15961 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t7775 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8192 - t11659 + t11070 - t11781 - t8190 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11404 - t11417;
    (t15961, t15966)
}
