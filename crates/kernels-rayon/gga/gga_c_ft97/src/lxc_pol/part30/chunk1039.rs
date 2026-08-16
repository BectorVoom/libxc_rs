//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1039/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1039(t108447: f64, t1127: f64, t123028: f64, t142832: f64, t150496: f64, t150752: f64, t150844: f64, t150846: f64, t150849: f64, t150858: f64, t150864: f64, t27521: f64, t27616: f64, t27638: f64, t27647: f64, t27672: f64, t33372: f64, t33374: f64, t33375: f64, t33403: f64, t33404: f64, t33418: f64, t35462: f64, t3789: f64, t6037: f64, t6828: f64, t709: f64, t7464: f64, t7470: f64, t7477: f64, t92354: f64) -> f64 {
    let t150875 = 0.39525571512470170088e-4_f64 * t7477 * t142832 * t7464 * t1127 * t709 - 0.68116566383613497688e-3_f64 * t27521 * t7470 * t150496 - 0.13784064983740990796e-3_f64 * t33418 * t150752 + 0.28200083969358461043e-4_f64 * t150844 * t150846 - 0.1443087735596363459e-7_f64 * t3789 * t150849 * t35462 * t709 - 0.40859909362962962964e0_f64 * t33372 * t6828 * t33374 + 0.15322466011111111111e0_f64 * t150858 * t33375 - 0.22705522127871165896e-3_f64 * t108447 * t33404 * t27638 + 0.31680880081247724282e-4_f64 * t27616 * t150864 * t6037 + 0.23022991505793434254e-7_f64 * t123028 * t92354 * t33403 * t27672 - 0.22705522127871165896e-3_f64 * t108447 * t33404 * t27647;
    t150875
}
