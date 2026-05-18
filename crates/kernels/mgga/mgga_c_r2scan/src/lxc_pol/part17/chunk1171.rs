//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1171/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1171<F: Float>(t11649: F, t30792: F, t12529: F, t6395: F, t3281: F, t9273: F, t11696: F, t40075: F, t10710: F, t10728: F, t27955: F, t11699: F, t39961: F) -> (F, F, F, F, F, F) {
    let t43273 = t30792 * t11649;
    let t43275 = t6395 * t12529;
    let t43277 = t3281 * t9273;
    let t43281 = t40075 * t11696;
    let t43284 = t10728 * t10710 * t27955;
    let t43286 = t39961 * t11699;
    (t43273, t43275, t43277, t43281, t43284, t43286)
}
