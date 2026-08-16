//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 991/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk991(t24465: f64, t28899: f64, t16524: f64, t33659: f64, t2039: f64, t28017: f64, t3941: f64, t33656: f64, t7769: f64, t94170: f64, t127630: f64, t8657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127679 = 27.0_f64 * t24465 * t28899;
    let t127681 = 54.0_f64 * t16524 * t33659;
    let t127684 = 27.0_f64 * t3941 * t2039 * t28017;
    let t127686 = 54.0_f64 * t16524 * t33656;
    let t127688 = 54.0_f64 * t94170 * t7769;
    let t127690 = 54.0_f64 * t127630 * t8657;
    (t127679, t127681, t127684, t127686, t127688, t127690)
}
