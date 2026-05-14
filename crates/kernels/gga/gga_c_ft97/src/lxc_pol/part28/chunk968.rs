//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 968/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk968<F: Float>(t1008: F, t1554: F, t3404: F, t420: F, t71: F, t7195: F, t145468: F, t32764: F, t104819: F, t138739: F, t138874: F, t138888: F, t138891: F, t145154: F, t145163: F, t145376: F, t145416: F, t145419: F, t145471: F, t147234: F, t147243: F, t147298: F, t23701: F, t23711: F, t23825: F, t23832: F, t23842: F, t32140: F, t32241: F, t32767: F, t32774: F, t3379: F, t34873: F, t378: F, t379: F, t94530: F) -> (F,) {
    let t147376 = t1554 * t1008;
    let t147390 = t7195 * t420 * t71 * t3404;
    let t147395 = t32764 * t145468;
    let t147402 = 0.80027204934668021496e-1 * t32767 * t32140 * t378 * t3379 - 0.12004080740200203224e0 * t32774 * t32140 * t378 * t3404 + 0.6041940442683716741e-1 * t23711 * t145416 - 0.40279602951224778273e-1 * t23711 * t145419 - 0.6041940442683716741e-1 * t94530 * t147234 - 0.6041940442683716741e-1 * t23701 * t145416 - 0.53351469956445347664e-1 * t147243 * t145163 + 0.53351469956445347664e-1 * t138739 * t32241 * t147376 * t379 + 0.6041940442683716741e-1 * t23701 * t145154 - 0.80027204934668021496e-1 * t138888 - 0.18125821328051150223e0 * t104819 * t34873 - 0.18125821328051150223e0 * t23832 * t147298 + 0.18125821328051150223e0 * t23842 * t147390 - 0.1422705865505209271e0 * t32764 * t145471 + 0.17783823318815115888e-1 * t147395 - 0.18125821328051150223e0 * t23825 * t147390 + 0.70628613596813898777e-2 * t138874 * t145376 - 0.24167761770734866964e0 * t138891;
    (t147402,)
}
