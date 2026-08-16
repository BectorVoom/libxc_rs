//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 454/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk454(t1797: f64, t377: f64, t1767: f64, t359: f64, t376: f64, t1170: f64, t1130: f64, t1773: f64, t375: f64, t1747: f64, t355: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1798 = t1797 * t377;
    let t1800 = t359 * t1767;
    let t1801 = t376 * t1800;
    let t1802 = t1170 * t1801;
    let t1804 = t1130 * t1773;
    let t1805 = t376 * t1804;
    let t1806 = t375 * t1805;
    let t1808 = t1747 * t355;
    let t1809 = t1808 * t381;
    (t1798, t1800, t1801, t1802, t1804, t1805, t1806, t1808, t1809)
}
