//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 347/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk347(t1724: f64, t1725: f64, t1695: f64, t1699: f64, t45: f64, t625: f64, t630: f64, t1718: f64, t1710: f64, t1715: f64, t1722: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1726 = t1724 * t1725;
    let t1729 = 0.92708333333333333333e-2_f64 * t1695;
    let t1731 = -t1729 - 0.92708333333333333333e-2_f64 * t1699;
    let t1735 = t45 * t625;
    let t1736 = t630 * t630;
    let t1737 = 1.0_f64 / t1736;
    let t1739 = 0.301925e0_f64 * t1695;
    let t1742 = 0.16557e0_f64 * t1718;
    let t1744 = 0.258925e1_f64 * t1710 - t1739 - 0.301925e0_f64 * t1699 + 0.16504875e0_f64 * t1715 - t1742 - 0.16557e0_f64 * t1722;
    (t1726, t1731, t1735, t1736, t1737, t1744)
}
