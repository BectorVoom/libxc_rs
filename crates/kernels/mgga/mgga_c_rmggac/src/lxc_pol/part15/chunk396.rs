//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 396/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk396<F: Float>(t118: F, t2402: F, t2066: F, t2087: F, t2382: F, t2384: F, t2386: F, t2388: F, t2390: F, t2394: F, t2396: F, t2398: F, t2400: F) -> F {
    let t2403 = t118 * t2402;
    let t2405 = F::new(0.2993560425465952141e-1) * t2382 - F::new(0.44903406381989282115e-1) * t2384 - F::new(0.14967802127329760705e-1) * t2386 - t2066 - F::new(0.10227998120342003148e-1) * t2388 + F::new(0.13637330827122670864e-1) * t2390 + F::new(0.34093327067806677161e-2) * t2394 + t2087 + F::new(0.59871208509319042821e-1) * t2396 - F::new(0.59871208509319042821e-1) * t2398 - F::new(0.39914139006212695214e-1) * t2400 + F::new(0.19957069503106347607e-1) * t2403;
    t2405
}
