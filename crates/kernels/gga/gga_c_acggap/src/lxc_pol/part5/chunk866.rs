//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 866/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk866<F: Float>(t12357: F, t381: F, t452: F, t1258: F, t980: F, t12235: F, t3036: F, t2937: F, t929: F, t1160: F, t3065: F, t930: F) -> (F, F, F, F, F) {
    let t12360 = F::new(0.65854491829355115987e0) * t381 * t452 * t12357;
    let t12385 = t980 * t1258;
    let t12395 = F::new(0.23707617058567841754e2) * t3036 * t452 * t12235;
    let t12401 = t2937 * t929;
    let t12410 = t1160 * t3065 * t930;
    (t12360, t12385, t12395, t12401, t12410)
}
