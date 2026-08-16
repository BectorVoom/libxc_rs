//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 358/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk358(t1783: f64, t1785: f64, t1310: f64, t1765: f64, t1771: f64, t1773: f64, t1778: f64, t664: f64, t667: f64, t1333: f64, t721: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1786 = t1783 * t1785;
    let t1787 = t1310 * t1786;
    let t1790 = 0.5397236614853195164e-1_f64 * t1765 * t664 + t1771 + 0.17990788716177317213e-1_f64 * t1773 * t1778 - 0.5397236614853195164e-1_f64 * t1773 * t1787;
    let t1791 = 1.0_f64 / t667;
    let t1792 = t1790 * t1791;
    let t1795 = t1333 * t721;
    let t1796 = 0.16581944444444444444e-2_f64 * t1795;
    let t1797 = 1.0_f64 / t690;
    (t1786, t1787, t1790, t1791, t1792, t1795, t1796, t1797)
}
