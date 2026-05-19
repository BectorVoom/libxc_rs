//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 878/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk878<F: Float>(t158: F, t165: F, t173: F, t23238: F, t23249: F, t23251: F, t23253: F, t28645: F, t28648: F, t28651: F, t28654: F, t28657: F, t28660: F, t28663: F, t5089: F, t5168: F) -> F {
    let t28671 = -F::cast_from(0.39624999999999999999e-2_f64) * t23238 - F::new(0.21078e-1) * t158 * t28645 + F::new(0.4755e-2) * t165 * t28648 + F::new(0.30247875e-4) * t173 * t28651 + F::new(0.317e-2) * t165 * t28654 + F::new(0.403305e-4) * t173 * t28657 + F::new(0.7925e-3) * t165 * t28660 + F::cast_from(0.46615850170166761884e-3_f64) * t5168 * t28663 - F::cast_from(0.71734315950379065738e-1_f64) * t5089 * t28663 + F::new(0.14052e-1) * t23249 - F::new(0.4684e-2) * t23251 - F::new(0.28104e-1) * t23253;
    t28671
}
