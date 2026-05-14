//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 825/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk825<F: Float>(t3519: F, t663: F, t685: F, t1084: F, t7489: F, t5522: F, t5745: F, t7357: F, t7500: F, t9148: F, t9163: F, t228: F, t1987: F, t3626: F, t2849: F, t2865: F) -> (F, F, F, F, F, F, F) {
    let t9334 = t3519 * t663;
    let t9336 = 1.0 * t9334 * t685;
    let t9338 = 2.0 * t7489 * t1084;
    let t9343 = -t5745 + 0.23744444444444444444e-1 * t5522 + 0.47488888888888888888e-1 * t7357 - t7500 - 0.17808333333333333333e-1 * t9148 + 0.53425e-1 * t9163;
    let t9345 = 0.621814e-1 * t9343 * t228;
    let t9347 = 0.17315859105681463759e2 * t1987 * t3626;
    let t9348 = t2865 * t2849;
    (t9334, t9336, t9338, t9343, t9345, t9347, t9348)
}
