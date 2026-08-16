//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 899/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk899(t17872: f64, t4950: f64, t13491: f64, t13520: f64, t13527: f64, t17801: f64, t17807: f64, t17809: f64, t17813: f64, t17819: f64, t17821: f64, t17825: f64, t17828: f64, t17833: f64, t17838: f64, t17843: f64, t17847: f64, t17851: f64, t17854: f64, t17856: f64, t17859: f64, t17865: f64, t17870: f64, t3762: f64, t3766: f64, t3774: f64, t4949: f64, t4952: f64, t9545: f64) -> f64 {
    let t17873 = t4950 * t17872;
    let t17876 = 0.43649539115179804188e-3_f64 * t3774 * t17801 * t3762 - 0.16340680006645994456e-5_f64 * t17807 * t17809 * t3762 - 0.13784064983740990796e-3_f64 * t4949 * t17813 * t4952 + 0.32054706583615839486e-5_f64 * t17819 * t17821 + 0.27568129967481981592e-4_f64 * t17825 * t17828 + 0.91830411319857336049e-5_f64 * t17833 * t17828 - 0.27568129967481981592e-3_f64 * t17838 * t17843 + 0.13519760450715832853e-3_f64 * t17847 * t13527 - 0.67598802253579164263e-4_f64 * t17851 * t9545 - 0.33776098467676728323e-5_f64 * t17854 * t17856 + 8.0_f64 * t3766 * t13491 * t17859 + 0.13784064983740990796e-3_f64 * t13520 * t17865 + 0.13784064983740990796e-3_f64 * t17870 * t17873;
    t17876
}
