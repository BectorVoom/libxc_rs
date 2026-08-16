//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1073/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1073(t7937: f64, t8764: f64, t2163: f64, t7741: f64, t651: f64, t1518: f64, t8756: f64, t7586: f64, t7742: f64, t1502: f64, t1911: f64, t33655: f64, t33659: f64, t33661: f64, t33664: f64, t33666: f64, t33669: f64, t7746: f64, t8761: f64) -> (f64, f64, f64) {
    let t34424 = t8764 * t7937;
    let t34428 = t2163 * t7741;
    let t34429 = t651 * t34428;
    let t34431 = t8756 * t1518;
    let t34434 = t7586 * t7742;
    let t34438 = -t1502 * t8756 + t1911 * t8761 - 2.0_f64 * t34431 * t651 - 2.0_f64 * t7586 * t7746 - t33655 + t33659 + 3.0_f64 * t33661 - t33664 - t33666 + t33669 - t34424 - 2.0_f64 * t34429 - 2.0_f64 * t34434;
    (t34428, t34431, t34438)
}
