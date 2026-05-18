//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1049/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1049<F: Float>(t136531: F, t136560: F, t136920: F, t136922: F, t136926: F, t136930: F, t136967: F, t136995: F, t145372: F, t145376: F, t145379: F, t145382: F, t1624: F, t25688: F, t25774: F, t25779: F, t25780: F, t25802: F, t3057: F, t32251: F, t32252: F, t32253: F, t32295: F, t32304: F, t36364: F, t37481: F, t92354: F, sigma0: F) -> F {
    let t145409 = F::new(0.26043295784446077722e-6) * t136967 * t145376 + F::new(0.25845121844514357744e-4) * t32304 * t145379 + F::new(0.28200083969358461042e-4) * t136995 * t145382 + F::new(0.13784064983740990797e-3) * t32295 * t145372 - F::new(0.17816121467177433866e-2) * t136930 * t25780 + F::new(0.79202200203119310706e-6) * t136560 * t36364 * t25802 + F::new(0.6595632919850939344e-7) * t1624 * t92354 * t37481 * sigma0 * t36364 * t25774 + F::new(0.79202200203119310706e-5) * t136926 * t36364 * t25779 - F::new(0.45497819271775541929e-4) * t136920 * t136922 * t25688 + F::new(0.22705522127871165896e-3) * t136531 + F::new(0.10338048737805743097e-3) * t32251 * t32252 * t32253 * t3057;
    t145409
}
