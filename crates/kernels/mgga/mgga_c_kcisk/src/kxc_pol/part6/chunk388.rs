//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 388/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk388<F: Float>(t2521: F, t706: F, t1421: F, t1875: F, t2399: F, t2514: F, t2518: F, t456: F, t604: F) -> (F, F) {
    let t2522 = t706 * t2521;
    let t2527 = t1875 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t2514 + F::cast_from(0.1478346675e-2_f64) * t456 * t2518 - F::new(0.98556445e-3) * t456 * t2522 - F::new(4.0) * t604 * t2399;
    (t2522, t2527)
}
