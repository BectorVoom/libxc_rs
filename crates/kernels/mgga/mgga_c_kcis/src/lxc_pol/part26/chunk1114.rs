//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1114/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1114<F: Float>(t12825: F, t7978: F, t7980: F, t12651: F, t1616: F, t1370: F, t27614: F, t94588: F, t1444: F, t4314: F, t27651: F, t7964: F, t2257: F, t2259: F, t44682: F, t27733: F) -> (F, F, F, F, F, F, F, F) {
    let t95001 = t7978 * t12825 * t7980;
    let t95009 = t12651 * t1616;
    let t95024 = t1370 * t27614;
    let t95088 = 0.51588271604938271604e-3 * t94588;
    let t95103 = t4314 * t1444;
    let t95137 = t7964 * t27651;
    let t95168 = 0.12871334876543209877e-3 * t2257 * t44682 * t2259;
    let t95271 = 2.0 * t27733;
    (t95001, t95009, t95024, t95088, t95103, t95137, t95168, t95271)
}
