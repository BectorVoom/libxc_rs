//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 879/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk879(t2005: f64, t2009: f64, t2034: f64, t3025: f64, t7513: f64, t7653: f64, t7720: f64, t7743: f64, t7780: f64, t7782: f64, t7786: f64, t7790: f64, t7792: f64, t7795: f64, t7798: f64, t7800: f64, t7807: f64, t7812: f64, t813: f64, t8766: f64, t8770: f64, t8775: f64, t8789: f64, t8793: f64) -> f64 {
    let t8796 = 0.59584149919750711116e-1_f64 * t7720 - 0.11916829983950142223e0_f64 * t7743 - 0.14300195980740170668e1_f64 * t3025 * t7653 - 0.12269736305254639896e2_f64 * t813 * t8766 - 0.15889106645266856297e0_f64 * t7513 * t8770 + 0.23833659967900284446e0_f64 * t8775 * t2034 + 0.59584149919750711116e-1_f64 * t7780 + 0.29792074959875355558e-1_f64 * t7782 - 0.89376224879626066674e-1_f64 * t7786 - 0.29792074959875355558e-1_f64 * t7790 - 0.59584149919750711116e-1_f64 * t7792 + 0.11916829983950142223e0_f64 * t7795 - 0.2698205900461089792e0_f64 * t7798 + 0.51123901271894332902e0_f64 * t7800 + 0.76685851907841499352e0_f64 * t7807 - 0.76685851907841499352e0_f64 * t7812 - 0.71500979903700853338e0_f64 * t8789 * t2009 + 0.21450293971110256002e1_f64 * t8793 * t2005;
    t8796
}
