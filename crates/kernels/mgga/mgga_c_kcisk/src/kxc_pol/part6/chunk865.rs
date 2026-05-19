//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 865/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk865<F: Float>(t16541: F, t8550: F, t1664: F, t28357: F, t4742: F, t2382: F, t8573: F, t4704: F, t22801: F, t2381: F, t10696: F, t28341: F) -> (F, F, F, F, F) {
    let t28461 = F::new(6.0) * t16541 * t8550;
    let t28462 = t28357 * t1664;
    let t28464 = F::new(6.0) * t4742 * t28462;
    let t28465 = t2382 * t8573;
    let t28467 = F::new(6.0) * t4704 * t28465;
    let t28468 = t22801 * t2381;
    let t28470 = F::cast_from(0.48245472966453314466e2_f64) * t4742 * t28468;
    let t28471 = t10696 * t28341;
    (t28461, t28464, t28467, t28470, t28471)
}
