//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1445/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1445<F: Float>(t13866: F, t546: F, t25767: F, t6087: F, t25214: F, t545: F, t6091: F, t1616: F, t2207: F, t2526: F, t785: F, t6263: F, t910: F, t20954: F, t2605: F, t5143: F, t7313: F) -> (F, F, F, F, F, F) {
    let t27217 = t546 * t13866;
    let t27219 = t27217 * t25767 * t6087;
    let t27222 = t545 * t6091 * t25214;
    let t27228 = t2207 * t785 * t1616 * t2526;
    let t27229 = 0.6112917064160653851e0 * t27228;
    let t27232 = t2207 * t785 * t6263 * t910;
    let t27234 = t20954 * t2605;
    let t27242 = t7313 * t5143;
    (t27219, t27222, t27229, t27232, t27234, t27242)
}
