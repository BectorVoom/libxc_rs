//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 581/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk581(t27552: f64, t6023: f64, t13520: f64, t1417: f64, t1701: f64, t24276: f64, t24324: f64, t24346: f64, t27487: f64, t27495: f64, t27500: f64, t27502: f64, t27507: f64, t27512: f64, t27516: f64, t27521: f64, t27524: f64, t27527: f64, t27529: f64, t27534: f64, t27540: f64, t27543: f64, t27546: f64, t27548: f64, t3730: f64, t3734: f64, t3755: f64, t6045: f64, t6055: f64) -> (f64, f64) {
    let t27553 = t6023 * t27552;
    let t27556 = 0.46509801892875584e-2_f64 * t27487 * t3734 + 0.38731446812548799881e-3_f64 * t27487 * t3755 + 0.23254900946437792e-1_f64 * t24346 * t3730 - 0.11854761295685025975e-1_f64 * t1417 * t1701 * t27495 + 0.12768721675925925926e-1_f64 * t27500 * t27502 + 0.17024962234567901235e-1_f64 * t6055 * t27507 + 0.12768721675925925926e-1_f64 * t6055 * t27512 - 0.11491849508333333333e0_f64 * t24324 * t27516 + 0.90822088511484663582e-3_f64 * t27521 * t27524 + 0.51690243689028715487e-4_f64 * t27527 * t6023 * t27529 + 0.7423383944657264111e-4_f64 * t24276 * t27534 + 0.74233839446572641111e-4_f64 * t24276 * t27540 - 0.25845121844514357744e-4_f64 * t13520 * t27543 + 0.76612330055555555556e-1_f64 * t27546 * t6045 * t27548 - 0.25845121844514357744e-4_f64 * t13520 * t27553;
    (t27553, t27556)
}
