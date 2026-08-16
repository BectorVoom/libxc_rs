//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 831/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk831(t355: f64, t6680: f64, t377: f64, t1801: f64, t5083: f64, t1797: f64, t1805: f64, t359: f64, t6486: f64, t376: f64, t3464: f64, t3338: f64, t6491: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6681 = t6680 * t355;
    let t6682 = t6681 * sigma0;
    let t6683 = t6682 * t377;
    let t6685 = t5083 * t1801;
    let t6687 = t1797 * t1805;
    let t6689 = t359 * t6486;
    let t6690 = t376 * t6689;
    let t6691 = t3464 * t6690;
    let t6693 = t3338 * t6491;
    (t6682, t6683, t6685, t6687, t6689, t6690, t6691, t6693)
}
