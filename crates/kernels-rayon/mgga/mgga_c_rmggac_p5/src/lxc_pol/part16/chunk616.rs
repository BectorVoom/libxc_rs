//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 616/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk616(t674: f64, t8675: f64, t1997: f64, t2004: f64, t2412: f64, t2007: f64, t1987: f64, t1990: f64, t457: f64, t589: f64, t201: f64, t1979: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8676 = t8675 * t674;
    let t8677 = t8676 * t1997;
    let t8679 = t2412 * t2004;
    let t8681 = t2412 * t2007;
    let t8683 = t2412 * t1987;
    let t8685 = t2412 * t1990;
    let t8687 = t589 * t457;
    let t8688 = t8687 * t201;
    let t8690 = t8688 * t1979 * t1982;
    (t8676, t8677, t8679, t8681, t8683, t8685, t8687, t8688, t8690)
}
