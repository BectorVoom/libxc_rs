//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1242/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1242<F: Float>(t27079: F, t20575: F, t7606: F, t13866: F, t546: F, t25214: F, t545: F, t6091: F, t1616: F, t2207: F, t2526: F, t785: F, t6263: F, t910: F, t20954: F, t2605: F) -> (F, F, F, F, F, F, F) {
    let t27080 = 0.87816964854445047168e-1 * t27079;
    let t27198 = t20575 * t7606;
    let t27217 = t546 * t13866;
    let t27222 = t545 * t6091 * t25214;
    let t27228 = t2207 * t785 * t1616 * t2526;
    let t27229 = 0.6112917064160653851e0 * t27228;
    let t27232 = t2207 * t785 * t6263 * t910;
    let t27234 = t20954 * t2605;
    (t27080, t27198, t27217, t27222, t27229, t27232, t27234)
}
