//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 899/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk899<F: Float>(t17872: F, t4950: F, t13491: F, t13520: F, t13527: F, t17801: F, t17807: F, t17809: F, t17813: F, t17819: F, t17821: F, t17825: F, t17828: F, t17833: F, t17838: F, t17843: F, t17847: F, t17851: F, t17854: F, t17856: F, t17859: F, t17865: F, t17870: F, t3762: F, t3766: F, t3774: F, t4949: F, t4952: F, t9545: F) -> F {
    let t17873 = t4950 * t17872;
    let t17876 = F::cast_from(0.43649539115179804188e-3_f64) * t3774 * t17801 * t3762 - F::cast_from(0.16340680006645994456e-5_f64) * t17807 * t17809 * t3762 - F::cast_from(0.13784064983740990796e-3_f64) * t4949 * t17813 * t4952 + F::cast_from(0.32054706583615839486e-5_f64) * t17819 * t17821 + F::cast_from(0.27568129967481981592e-4_f64) * t17825 * t17828 + F::cast_from(0.91830411319857336049e-5_f64) * t17833 * t17828 - F::cast_from(0.27568129967481981592e-3_f64) * t17838 * t17843 + F::cast_from(0.13519760450715832853e-3_f64) * t17847 * t13527 - F::cast_from(0.67598802253579164263e-4_f64) * t17851 * t9545 - F::cast_from(0.33776098467676728323e-5_f64) * t17854 * t17856 + F::new(8.0) * t3766 * t13491 * t17859 + F::cast_from(0.13784064983740990796e-3_f64) * t13520 * t17865 + F::cast_from(0.13784064983740990796e-3_f64) * t17870 * t17873;
    t17876
}
