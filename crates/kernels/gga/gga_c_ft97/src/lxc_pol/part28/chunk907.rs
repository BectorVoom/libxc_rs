//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 907/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk907<F: Float>(t25749: F, t32247: F, t101075: F, t136275: F, t136279: F, t136301: F, t136305: F, t136336: F, t136469: F, t145099: F, t25709: F, t25746: F, t25787: F, t25839: F, t32146: F, t32233: F, t32239: F, t34468: F, t6441: F, t93117: F) -> (F,) {
    let t145108 = t32247 * t25749;
    let t145120 = -0.23754828622903245155e-2 * t32146 * t136336 * t6441 + 0.29693535778629056444e-3 * t145099 + 0.51074886703703703704e-1 * t32247 * t136469 * t25709 - 0.85124811172839506173e-2 * t136279 + 0.68099848938271604939e-1 * t32247 * t25746 - 0.45497819271775541929e-4 * t136301 - 0.85124811172839506173e-2 * t145108 - 0.13623313276722699538e-2 * t93117 * t32233 * t25839 + 0.37842536879785276493e-4 * t136305 + 0.20434969915084049306e-2 * t101075 * t32233 * t25787 + 0.36398255417420433543e-3 * t32239 * t136275 * t34468;
    (t145120,)
}
