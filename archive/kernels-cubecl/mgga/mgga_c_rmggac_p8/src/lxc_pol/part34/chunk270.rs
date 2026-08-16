//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 270/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk270<F: Float>(t2123: F, t338: F, t118: F, t2055: F, t2058: F, t2062: F, t2066: F, t2071: F, t2076: F, t2082: F, t2087: F, t2088: F, t2090: F, t2092: F) -> (F, F, F) {
    let t2124 = t338 * t2123;
    let t2125 = t118 * t2124;
    let t2127 = F::cast_from(0.2993560425465952141e-1_f64) * t2055 - F::cast_from(0.44903406381989282115e-1_f64) * t2058 - F::cast_from(0.14967802127329760705e-1_f64) * t2062 - t2066 - F::cast_from(0.10227998120342003148e-1_f64) * t2071 + F::cast_from(0.13637330827122670864e-1_f64) * t2076 + F::cast_from(0.34093327067806677161e-2_f64) * t2082 + t2087 + F::cast_from(0.59871208509319042821e-1_f64) * t2088 - F::cast_from(0.59871208509319042821e-1_f64) * t2090 - F::cast_from(0.39914139006212695214e-1_f64) * t2092 + F::cast_from(0.19957069503106347607e-1_f64) * t2125;
    (t2124, t2125, t2127)
}
