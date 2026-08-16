//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 805/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk805(t8772: f64, t7616: f64, t7617: f64, t7622: f64, t7625: f64, t7629: f64, t7645: f64, t8219: f64, t8220: f64, t8221: f64, t8776: f64, t8780: f64, t8784: f64, t8788: f64, t8794: f64) -> f64 {
    let t9292 = 0.1528125e-1_f64 * t8772;
    let t9298 = t7616 - 0.80031500487063509014e-2_f64 * t7617 + 0.80031500487063509014e-2_f64 * t7622 - t7625 + t7629 + t8219 + t8220 - t8221 - t9292 + 0.10718504529517434243e-2_f64 * t8776 + 0.42874018118069736972e-3_f64 * t8780 - 0.15724046144802076034e-2_f64 * t8784 - 0.94344276868812456207e-3_f64 * t8788 - 0.62896184579208304138e-3_f64 * t8794 + t7645;
    t9298
}
