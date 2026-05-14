//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 858/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk858<F: Float>(t27552: F, t6023: F, t13520: F, t1417: F, t1701: F, t24276: F, t24324: F, t24346: F, t27487: F, t27495: F, t27500: F, t27502: F, t27507: F, t27512: F, t27516: F, t27521: F, t27524: F, t27527: F, t27529: F, t27534: F, t27540: F, t27543: F, t27546: F, t27548: F, t3730: F, t3734: F, t3755: F, t6045: F, t6055: F) -> (F,) {
    let t27553 = t6023 * t27552;
    let t27556 = 0.46509801892875584e-2 * t27487 * t3734 + 0.38731446812548799881e-3 * t27487 * t3755 + 0.23254900946437792e-1 * t24346 * t3730 - 0.11854761295685025975e-1 * t1417 * t1701 * t27495 + 0.12768721675925925926e-1 * t27500 * t27502 + 0.17024962234567901235e-1 * t6055 * t27507 + 0.12768721675925925926e-1 * t6055 * t27512 - 0.11491849508333333333e0 * t24324 * t27516 + 0.90822088511484663582e-3 * t27521 * t27524 + 0.51690243689028715487e-4 * t27527 * t6023 * t27529 + 0.7423383944657264111e-4 * t24276 * t27534 + 0.74233839446572641111e-4 * t24276 * t27540 - 0.25845121844514357744e-4 * t13520 * t27543 + 0.76612330055555555556e-1 * t27546 * t6045 * t27548 - 0.25845121844514357744e-4 * t13520 * t27553;
    (t27556,)
}
