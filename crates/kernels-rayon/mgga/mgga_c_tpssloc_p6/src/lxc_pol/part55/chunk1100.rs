//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1100/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1100(t32840: f64, t8344: f64, t232: f64, t4180: f64, t4181: f64, t30714: f64, t1516: f64, t8343: f64, t30698: f64, t30705: f64, t30722: f64, t32835: f64, t32838: f64) -> (f64, f64) {
    let t32841 = t32840 * t8344;
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32849 = -t30698 - 0.48447307312968469025e-2_f64 * t32835 - t30705 - 0.80745512188280781708e-3_f64 * t32838 + t32841 / 1536.0_f64 - t32845 / 1536.0_f64 - t30722 - t32847 / 384.0_f64;
    (t32844, t32849)
}
