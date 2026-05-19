//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 838/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk838<F: Float>(t71: F, t938: F, t420: F, t1301: F, t32296: F, t6427: F, t52: F, t7182: F, t22736: F, t22842: F, t32133: F, t32146: F, t32185: F, t32228: F, t32247: F, t32251: F, t32259: F, t32295: F, t32301: F, t34421: F, t34424: F, t34427: F, t34430: F, t6450: F, t931: F) -> (F, F, F, F, F, F) {
    let t34433 = t71 * t938;
    let t34434 = t420 * t34433;
    let t34435 = t1301 * t34434;
    let t34440 = t32296 * t6427;
    let t34444 = t52 * t7182 * t938;
    let t34450 = -F::cast_from(0.23254900946437792e-1_f64) * t32301 * t931 - F::new(2.0) * t34421 + F::cast_from(0.10338048737805743097e-3_f64) * t32251 * t34424 + F::cast_from(0.88910709717637694816e-2_f64) * t22842 * t34427 + F::cast_from(0.89080607335887169333e-3_f64) * t32146 * t34430 + F::cast_from(0.15322466011111111111e0_f64) * t32259 * t34435 - t32185 - F::cast_from(0.25537443351851851852e-1_f64) * t32247 * t6450 - F::cast_from(0.25845121844514357744e-4_f64) * t32295 * t34440 - F::cast_from(0.22227677429409423704e-2_f64) * t32228 * t34444 - F::cast_from(0.22979081259345929704e-6_f64) * t22736 * t32133 * t6427;
    (t34433, t34434, t34435, t34440, t34444, t34450)
}
