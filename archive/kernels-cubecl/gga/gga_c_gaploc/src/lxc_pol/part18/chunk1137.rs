//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1137/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1137<F: Float>(t2464: F, t2465: F, t587: F, t6417: F, t2487: F, t6428: F, t3178: F, t4625: F, t1407: F, t9279: F, t9555: F, t3193: F, t4634: F) -> (F, F, F, F, F, F) {
    let t30374 = F::cast_from(0.17041300423964777634e0_f64) * t587 * t2464 * t2465 * t6417;
    let t30378 = F::cast_from(0.17041300423964777634e0_f64) * t2487 * t2464 * t2465 * t6428;
    let t30379 = t4625 * t3178;
    let t30380 = F::cast_from(0.38342925953920749676e0_f64) * t30379;
    let t30381 = t1407 * t9279;
    let t30382 = F::cast_from(0.76685851907841499352e0_f64) * t30381;
    let t30387 = t1407 * t9555;
    let t30388 = F::cast_from(0.1022478025437886658e1_f64) * t30387;
    let t30404 = t4634 * t3193;
    (t30374, t30378, t30380, t30382, t30388, t30404)
}
