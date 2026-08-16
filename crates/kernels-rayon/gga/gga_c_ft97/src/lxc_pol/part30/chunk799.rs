//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 799/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk799(t33866: f64, t33959: f64, t33976: f64, t33871: f64, t33876: f64, t33956: f64, t33964: f64, t33969: f64, t33973: f64, t33981: f64, t33986: f64, t33990: f64) -> (f64, f64, f64, f64) {
    let t34042 = t33866 / 6.0_f64;
    let t34045 = 2.0_f64 / 3.0_f64 * t33959;
    let t34049 = t33976 / 3.0_f64;
    let t34052 = t34042 + t33871 / 6.0_f64 + t33876 - t33956 / 2.0_f64 - t34045 - 2.0_f64 / 3.0_f64 * t33964 - 6.0_f64 * t33969 + 4.0_f64 * t33973 + t34049 + t33981 / 3.0_f64 + 2.0_f64 * t33986 - t33990;
    (t34042, t34045, t34049, t34052)
}
