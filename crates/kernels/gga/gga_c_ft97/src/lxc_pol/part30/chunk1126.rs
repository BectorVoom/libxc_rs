//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1126/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1126<F: Float>(t150658: F, t7009: F, t153074: F, t800: F, t153080: F, t285: F, t142697: F, t142779: F, t142787: F, t150372: F, t150546: F, t153054: F, t153077: F, t153083: F, t153155: F, t153188: F, t153193: F, t153196: F, t153205: F, t153208: F, t19039: F, t28566: F, t31462: F, t33426: F, t33906: F, t33934: F, t33935: F, t33941: F, t33942: F, t35890: F, t4089: F, t7590: F, t82957: F) -> F {
    let t153210 = t7009 * t150658;
    let t153216 = t800 * t153074;
    let t153219 = t285 * t153080;
    let t153229 = F::cast_from(0.45306850413028723348e0_f64) * t33906 * t153054 + F::cast_from(0.3531430679840694939e-2_f64) * t153188 - F::cast_from(0.22653425206514361674e0_f64) * t31462 * t153155 - F::cast_from(0.24163653553615319118e1_f64) * t33906 * t153193 + F::cast_from(0.70628613596813898777e-2_f64) * t153196 - F::cast_from(0.42681175965156278132e0_f64) * t33934 * t150546 * t33935 + F::cast_from(0.64021763947734417198e0_f64) * t33941 * t150546 * t33942 - F::cast_from(0.80027204934668021493e-1_f64) * t153205 + F::cast_from(0.53351469956445347664e-1_f64) * t153208 - F::cast_from(0.24167761770734866964e0_f64) * t153210 + F::cast_from(0.17783823318815115888e-1_f64) * t142779 + F::cast_from(0.20527106943485609994e0_f64) * t82957 * t35890 + F::cast_from(0.10069900737806194568e-1_f64) * t142787 + F::cast_from(0.58778941170896004276e-1_f64) * t153216 * t153077 - F::cast_from(0.88168411756344006414e-1_f64) * t153219 * t153083 + F::cast_from(0.53351469956445347664e-1_f64) * t142697 * t33426 * t150372 * t28566 + F::cast_from(0.20527106943485609994e0_f64) * t19039 * t7590 * t4089;
    t153229
}
