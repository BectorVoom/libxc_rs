//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 366/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk366<F: Float>(t118: F, t2124: F, t2055: F, t2058: F, t2062: F, t2066: F, t2071: F, t2076: F, t2082: F, t2087: F, t2088: F, t2090: F, t2092: F) -> F {
    let t2125 = t118 * t2124;
    let t2127 = F::new(0.2993560425465952141e-1) * t2055 - F::new(0.44903406381989282115e-1) * t2058 - F::new(0.14967802127329760705e-1) * t2062 - t2066 - F::new(0.10227998120342003148e-1) * t2071 + F::new(0.13637330827122670864e-1) * t2076 + F::new(0.34093327067806677161e-2) * t2082 + t2087 + F::new(0.59871208509319042821e-1) * t2088 - F::new(0.59871208509319042821e-1) * t2090 - F::new(0.39914139006212695214e-1) * t2092 + F::new(0.19957069503106347607e-1) * t2125;
    t2127
}
