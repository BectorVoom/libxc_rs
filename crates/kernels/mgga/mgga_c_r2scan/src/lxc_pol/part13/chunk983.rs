//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 983/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk983<F: Float>(t1044: F, t3424: F, t3685: F, t885: F, t4176: F, t986: F, t3270: F, t3269: F, t1108: F, t2449: F, t1065: F, t983: F) -> (F, F, F, F, F, F, F) {
    let t11537 = t3424 * t1044;
    let t11538 = t3685 * t885;
    let t11539 = t4176 * t986;
    let t11540 = t3270 * t11539;
    let t11541 = t3269 * t11540;
    let t11542 = t11541 / F::new(4.0);
    let t11543 = t2449 * t1108;
    let t11544 = t1065 * t983;
    (t11537, t11538, t11540, t11541, t11542, t11543, t11544)
}
