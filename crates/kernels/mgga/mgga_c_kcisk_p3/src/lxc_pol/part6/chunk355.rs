//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 355/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk355<F: Float>(t2225: F, t457: F, t1419: F, t1421: F, t2110: F, t2218: F, t2222: F, t338: F, t456: F) -> (F, F) {
    let t2226 = t457 * t2225;
    let t2231 = t1419 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t2218 + F::cast_from(0.1478346675e-2_f64) * t456 * t2222 - F::new(0.98556445e-3) * t456 * t2226 - F::new(4.0) * t338 * t2110;
    (t2226, t2231)
}
