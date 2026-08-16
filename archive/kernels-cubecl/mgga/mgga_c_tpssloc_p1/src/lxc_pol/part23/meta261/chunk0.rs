//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 924/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk924<F: Float>(t5234: F, t5245: F, t12283: F, t6396: F, t1362: F, t19815: F, t3799: F, t6417: F, t6422: F, t16336: F, t1831: F, t3866: F, t6427: F) -> (F, F, F, F, F, F, F) {
    let t19876 = t5234 * t5245;
    let t19879 = t12283 * t6396;
    let t19904 = t19815 * t1362;
    let t19915 = t3799 * t6417;
    let t19917 = t3799 * t6422;
    let t19933 = t16336 * t1831;
    let t19940 = t3866 * t6427;
    (t19876, t19879, t19904, t19915, t19917, t19933, t19940)
}
