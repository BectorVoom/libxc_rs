//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 304/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk304(t1004: f64, t1011: f64, t1014: f64, t1017: f64, t1425: f64, t1535: f64, t1813: f64, t1814: f64, t1815: f64, t1816: f64, t1817: f64, t1819: f64, t1835: f64, t436: f64, t948: f64, t975: f64, t982: f64) -> f64 {
    let t1838 = t948 - t975 + 0.186546e0_f64 * t1425 * t1535 + t1813 - t1814 + t1815 - t1816 - t1817 + t982 - 0.31091e-1_f64 * t1819 * t1004 + 0.93273e-1_f64 * t436 * t1835 + t1011 + t1014 + t1017;
    t1838
}
