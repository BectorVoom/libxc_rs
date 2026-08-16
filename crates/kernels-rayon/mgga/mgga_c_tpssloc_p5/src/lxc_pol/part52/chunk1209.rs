//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1209/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1209(t235: f64, t32849: f64, t1499: f64, t226: f64, t30675: f64, t30683: f64, t32821: f64, t32825: f64, t32829: f64, t32831: f64, t812: f64, t8360: f64) -> (f64, f64) {
    let t32850 = t235 * t32849;
    let t32852 = t1499 * t8360 + t226 * t32850 - t32831 * t812 - t30675 - t30683 - t32821 - t32825 + t32829;
    (t32850, t32852)
}
