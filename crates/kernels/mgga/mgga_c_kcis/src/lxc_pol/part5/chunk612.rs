//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 612/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk612<F: Float>(t3944: F, t473: F, t1341: F, t187: F, t4114: F, t1588: F, t1591: F, t1590: F, t625: F, t609: F, t109: F, t494: F, t209: F, t617: F, t612: F, t1369: F, t25: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4373 = t473 * t3944;
    let t4381 = t187 * t1341;
    let t4399 = 0.38691203703703703703e-3 * t4114;
    let t4409 = t1588 * t1591;
    let t4413 = 1.0 / t1590 / t625;
    let t4414 = t609 * t4413;
    let t4420 = t109 * t494;
    let t4422 = t209 * t4420 * t617;
    let t4424 = t612 * t4422 / 864.0;
    let t4425 = t25 * t1369;
    (t4373, t4381, t4399, t4409, t4413, t4414, t4422, t4424, t4425)
}
