//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 751/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk751<F: Float>(t231: F, t2428: F, t13395: F, t13402: F, t13520: F, t24261: F, t24265: F, t24266: F, t24270: F, t24276: F, t24280: F, t24283: F, t24289: F, t24291: F, t24295: F, t24299: F, t24302: F, t24306: F, t24307: F, t24311: F, t24315: F, t24324: F, t3759: F, t3766: F, t3774: F, t6023: F, t6034: F, t6035: F, t6043: F, t6045: F, t6055: F) -> (F, F) {
    let t24325 = t231 * t2428;
    let t24329 = 4.0 * t3766 * t24261 - 0.89080607335887169332e-3 * t24265 * t24266 - 0.44540303667943584666e-4 * t6034 * t6035 * t24270 + 0.14846767889314528222e-3 * t24276 * t24280 + 0.46509801892875584e-2 * t3759 * t24283 + t24289 + 0.12768721675925925926e-1 * t6055 * t24291 - 0.6384360837962962963e-2 * t6055 * t24295 - 0.85124811172839506173e-2 * t6055 * t24299 - 0.51690243689028715488e-4 * t13520 * t24302 - 0.51789017496114396277e-5 * t24306 * t24307 - 0.1721820212247325051e-5 * t3774 * t24311 * t13402 + 0.38306165027777777778e-1 * t6043 * t6045 * t24315 - 0.51690243689028715488e-5 * t3774 * t6023 * t13395 - 0.11491849508333333333e0 * t24324 * t6045 * t24325;
    (t24325, t24329)
}
