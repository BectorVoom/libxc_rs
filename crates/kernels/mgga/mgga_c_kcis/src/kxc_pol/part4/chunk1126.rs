//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1126/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1126<F: Float>(t1098: F, t4627: F, t2944: F, t4637: F, t4600: F, t3096: F, t4606: F, t3293: F, t10284: F, t10286: F, t10333: F, t10335: F, t1102: F, t14184: F, t14188: F, t14193: F, t14198: F, t14202: F, t14204: F, t14206: F, t14211: F, t14217: F, t14221: F, t14224: F, t14228: F, t14232: F) -> (F, F, F) {
    let t14235 = F::new(0.19711289e-2) * t1098 * t4627;
    let t14238 = t4637 * t2944;
    let t14239 = t4600 * t14238;
    let t14242 = t4606 * t3096;
    let t14243 = t3293 * t14242;
    let t14246 = F::new(0.19711289e-2) * t1102 * t14184 + F::new(0.98556445e-3) * t1102 * t14188 + F::cast_from(0.16426074166666666667e-2_f64) * t1102 * t14193 - F::cast_from(0.1478346675e-2_f64) * t1102 * t14198 - t14202 + t14204 - F::new(0.59133867e-2) * t1102 * t14206 - F::cast_from(0.295669335e-2_f64) * t1102 * t14211 - F::cast_from(0.65704296666666666667e-3_f64) * t10284 + F::cast_from(0.295669335e-2_f64) * t1102 * t14217 + F::cast_from(0.13140859333333333333e-2_f64) * t1102 * t14221 + F::cast_from(0.39422577999999999999e-2_f64) * t1102 * t14224 + F::cast_from(0.492782225e-3_f64) * t1102 * t14228 + t14232 - F::cast_from(0.8760572888888888889e-3_f64) * t10286 + t14235 + F::cast_from(0.13140859333333333334e-2_f64) * t10333 - F::cast_from(0.8760572888888888889e-3_f64) * t10335 - F::cast_from(0.7391733375e-3_f64) * t1102 * t14239 - F::cast_from(0.1478346675e-2_f64) * t1102 * t14243;
    (t14238, t14242, t14246)
}
