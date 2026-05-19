//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 809/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk809<F: Float>(t8679: F, t8685: F, t8690: F, t8796: F, t7702: F, t7706: F, t7712: F, t7714: F, t7719: F, t7722: F, t7724: F, t7726: F, t7728: F, t8173: F) -> (F, F, F, F) {
    let t38292 = F::cast_from(0.85129199786595678796e-5_f64) * t8679;
    let t38295 = F::cast_from(0.85129199786595678796e-5_f64) * t8685;
    let t38296 = F::cast_from(0.85129199786595678796e-5_f64) * t8690;
    let t38300 = F::cast_from(0.39914139006212695214e-1_f64) * t8796;
    let t38301 = -t38300 - t7702 - t7706 + t8173 - t7712 + t7714 - t7719 - t7722 + t7724 - t7726 - t7728;
    (t38292, t38295, t38296, t38301)
}
