//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1222/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1222<F: Float>(t3263: F, t3275: F, t40667: F, t3446: F, t37475: F, t970: F, t1065: F, t2526: F, t3270: F, t10667: F, t105: F, t2530: F, t97: F) -> (F, F, F, F) {
    let t40670 = F::new(3.0) / F::new(2.0) * t3275 * t3263 * t40667;
    let t40672 = t3446 * t37475 * t970;
    let t40676 = t1065 * t2526;
    let t40677 = t3270 * t40676;
    let t40679 = F::new(3.0) / F::new(2.0) * t10667 * t40677;
    let t40681 = t97 * t105 * t2530;
    (t40670, t40672, t40679, t40681)
}
