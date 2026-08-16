//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1105/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1105(t152815: f64, t24981: f64, t6317: f64, t4226: f64, t7584: f64, t33811: f64, t7512: f64, t7641: f64, t152770: f64, t152774: f64, t152779: f64, t152783: f64, t152788: f64, t152792: f64, t152797: f64, t152801: f64, t152804: f64, t152807: f64, t152810: f64, t152813: f64, t152817: f64, t152821: f64) -> (f64, f64, f64, f64) {
    let t152824 = t6317 * t24981 * t152815;
    let t152826 = t7584 * t4226;
    let t152829 = t33811 * t7512 * t7641 * t152826;
    let t152831 = 2.0_f64 / 3.0_f64 * t152770 - 2.0_f64 / 9.0_f64 * t152774 - t152779 + 2.0_f64 / 3.0_f64 * t152783 + t152788 / 12.0_f64 - 2.0_f64 / 3.0_f64 * t152792 + t152797 / 12.0_f64 + 2.0_f64 / 3.0_f64 * t152801 + 2.0_f64 / 3.0_f64 * t152804 - 4.0_f64 / 9.0_f64 * t152807 + 4.0_f64 / 9.0_f64 * t152810 - 4.0_f64 / 27.0_f64 * t152813 - 2.0_f64 / 9.0_f64 * t152817 - t152821 / 36.0_f64 - t152824 / 9.0_f64 + t152829 / 2.0_f64;
    (t152824, t152826, t152829, t152831)
}
