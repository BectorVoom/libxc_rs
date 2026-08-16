//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1122/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1122(t34345: f64, t7585: f64, t8525: f64, t7839: f64, t9637: f64, t30769: f64, t30773: f64, t30775: f64, t30777: f64, t34837: f64, t34840: f64, t34849: f64, t34851: f64, t34853: f64, t34856: f64, t37271: f64, t39525: f64, t39527: f64, t39534: f64, t39537: f64, t39540: f64) -> f64 {
    let t39545 = t7585 * t34345 * t8525;
    let t39547 = t7839 * t9637;
    let t39549 = -t39525 / 16.0_f64 - t34837 + t34840 - 7.0_f64 / 288.0_f64 * t39527 + 0.34299214494455789578e-2_f64 * t30769 + t37271 + 0.21437009059034868486e-3_f64 * t30773 - 0.85748036236139473944e-3_f64 * t30775 + 0.85748036236139473944e-3_f64 * t30777 - 0.11321313224257494744e-1_f64 * t34849 - 0.21437009059034868486e-3_f64 * t39534 - 0.21437009059034868486e-3_f64 * t39537 - 0.21437009059034868486e-3_f64 * t39540 + 0.80031500487063509016e-2_f64 * t34851 - 0.80031500487063509016e-2_f64 * t34853 + t34856 - 0.14291339372689912324e-3_f64 * t39545 - 0.10718504529517434243e-3_f64 * t39547;
    t39549
}
