//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2403/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2403(t324: f64, t68736: f64, t68756: f64, t300: f64, t1557: f64, t59979: f64, t17195: f64, t4396: f64, t1068: f64, t25845: f64, t4700: f64, t60874: f64, t68441: f64, t68706: f64, t68708: f64, t68710: f64, t68711: f64, t68715: f64, t68717: f64) -> (f64, f64, f64, f64, f64) {
    let t68758 = (t68736 + t68756) * t324;
    let t68760 = 0.19751673498613801407e-1_f64 * t300 * t68758;
    let t68762 = 3.0_f64 * t59979 * t1557;
    let t68764 = 3.0_f64 * t17195 * t4396;
    let t68765 = -t1068 * t4700 * t68711 + 6.0_f64 * t25845 * t4700 * t60874 - t68441 - t68706 + t68708 - t68710 - t68715 - t68717 + t68760 + t68762 + t68764;
    (t68758, t68760, t68762, t68764, t68765)
}
