//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1095/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1095(t30714: f64, t32844: f64, t1516: f64, t8343: f64, t30698: f64, t30705: f64, t30722: f64, t32835: f64, t32838: f64, t32841: f64, t235: f64, t1499: f64, t226: f64, t30675: f64, t30683: f64, t32821: f64, t32825: f64, t32829: f64, t32831: f64, t812: f64, t8360: f64) -> (f64, f64, f64) {
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32849 = -t30698 - 0.48447307312968469025e-2_f64 * t32835 - t30705 - 0.80745512188280781708e-3_f64 * t32838 + t32841 / 1536.0_f64 - t32845 / 1536.0_f64 - t30722 - t32847 / 384.0_f64;
    let t32850 = t235 * t32849;
    let t32852 = t1499 * t8360 + t226 * t32850 - t32831 * t812 - t30675 - t30683 - t32821 - t32825 + t32829;
    (t32849, t32850, t32852)
}
