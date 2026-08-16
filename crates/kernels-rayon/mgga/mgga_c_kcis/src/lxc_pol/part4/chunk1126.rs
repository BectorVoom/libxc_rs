//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1126/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1126(t1098: f64, t4627: f64, t2944: f64, t4637: f64, t4600: f64, t3096: f64, t4606: f64, t3293: f64, t10284: f64, t10286: f64, t10333: f64, t10335: f64, t1102: f64, t14184: f64, t14188: f64, t14193: f64, t14198: f64, t14202: f64, t14204: f64, t14206: f64, t14211: f64, t14217: f64, t14221: f64, t14224: f64, t14228: f64, t14232: f64) -> (f64, f64, f64) {
    let t14235 = 0.19711289e-2_f64 * t1098 * t4627;
    let t14238 = t4637 * t2944;
    let t14239 = t4600 * t14238;
    let t14242 = t4606 * t3096;
    let t14243 = t3293 * t14242;
    let t14246 = 0.19711289e-2_f64 * t1102 * t14184 + 0.98556445e-3_f64 * t1102 * t14188 + 0.16426074166666666667e-2_f64 * t1102 * t14193 - 0.1478346675e-2_f64 * t1102 * t14198 - t14202 + t14204 - 0.59133867e-2_f64 * t1102 * t14206 - 0.295669335e-2_f64 * t1102 * t14211 - 0.65704296666666666667e-3_f64 * t10284 + 0.295669335e-2_f64 * t1102 * t14217 + 0.13140859333333333333e-2_f64 * t1102 * t14221 + 0.39422577999999999999e-2_f64 * t1102 * t14224 + 0.492782225e-3_f64 * t1102 * t14228 + t14232 - 0.8760572888888888889e-3_f64 * t10286 + t14235 + 0.13140859333333333334e-2_f64 * t10333 - 0.8760572888888888889e-3_f64 * t10335 - 0.7391733375e-3_f64 * t1102 * t14239 - 0.1478346675e-2_f64 * t1102 * t14243;
    (t14238, t14242, t14246)
}
