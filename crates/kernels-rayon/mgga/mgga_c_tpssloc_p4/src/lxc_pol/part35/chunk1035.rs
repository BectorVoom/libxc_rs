//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1035/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1035(t21238: f64, t942: f64, t951: f64, t959: f64, t21093: f64, t21097: f64, t21099: f64, t21103: f64, t21105: f64, t21107: f64, t21365: f64, t21367: f64, t21369: f64, t21375: f64) -> (f64, f64) {
    let t21589 = t942 * t21238 * t951;
    let t21591 = 0.5848223622634646207e0_f64 * t959 * t21589;
    let t21592 = t21367 + t21375 + t21369 - t21093 + t21097 - t21591 + t21365 - t21099 - t21105 - t21107 - t21103;
    (t21591, t21592)
}
