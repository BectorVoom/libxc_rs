//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1030/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1030(t150664: f64, t3773: f64, t1701: f64, t27724: f64, t108897: f64, t123543: f64, t13443: f64, t150655: f64, t150659: f64, t150662: f64, t226: f64, t237: f64, t25: f64, t27711: f64, t27713: f64, t30671: f64, t33362: f64, t35462: f64, t36801: f64, t3723: f64, t3725: f64, t3762: f64, t3777: f64, t3782: f64, t5009: f64, t677: f64, t7446: f64) -> f64 {
    let t150665 = t150664 * t3773;
    let t150668 = t1701 * t27724;
    let t150684 = 0.88910709717637694816e-2_f64 * t123543 * t33362 + 0.13519760450715832853e-3_f64 * t3723 * t7446 * t226 * t3725 - 0.11854761295685025975e-1_f64 * t30671 * t150655 - 0.90822088511484663583e-3_f64 * t150659 - 0.11738898233082762229e-1_f64 * t150662 - 0.13784064983740990796e-3_f64 * t150665 * t3777 + 0.44455354858818847408e-2_f64 * t108897 * t150668 + 0.44455354858818847408e-2_f64 * t27711 * t1701 * t27713 - 0.44455354858818847408e-2_f64 * t13443 * t150668 - 0.16779431174156321371e-9_f64 * t677 * t237 * t5009 * t35462 * t25 * t3762 - 0.11854761295685025975e-1_f64 * t36801 * t3782;
    t150684
}
