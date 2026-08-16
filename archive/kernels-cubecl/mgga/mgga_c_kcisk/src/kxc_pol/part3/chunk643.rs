//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 643/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk643<F: Float>(t1596: F, t4350: F, t2028: F, t5439: F, t3182: F, t1065: F, t3462: F, t1156: F, t4569: F, t294: F, t1008: F, t195: F) -> (F, F, F, F, F, F) {
    let t9517 = t4350 * t1596;
    let t9726 = t5439 * t2028;
    let t10328 = F::cast_from(6.0_f64) * t3182;
    let t10329 = t1065 * t3462;
    let t10330 = F::cast_from(3.0_f64) * t10329;
    let t10331 = t1156 * t4569;
    let t10332 = t294 * t10331;
    let t10333 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t10332;
    let t10334 = t1008 * t1008;
    let t10335 = F::cast_from(1.0_f64) / t10334;
    let t10336 = t195 * t10335;
    (t9517, t9726, t10328, t10330, t10333, t10336)
}
