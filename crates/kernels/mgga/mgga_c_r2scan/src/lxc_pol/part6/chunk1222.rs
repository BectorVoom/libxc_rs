//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1222/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1222<F: Float>(t406: F, t5475: F, t2090: F, t4: F, t612: F, t615: F, t5845: F, t5998: F, t6032: F, t6001: F, t5902: F, t761: F, t2061: F, t1654: F, t2049: F, t597: F, t6044: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22569 = t406 * t5475;
    let t22574 = 0.8781774676543209876e-2 * t612 * t615 * t4 * t2090;
    let t22575 = t406 * t5845;
    let t22587 = t6032 * t5998;
    let t22589 = t6032 * t6001;
    let t22591 = t5902 * t761;
    let t22592 = t2061 * t22591;
    let t22595 = t1654 * t2049;
    let t22596 = t2061 * t22595;
    let t22602 = t597 * t6044;
    (t22569, t22574, t22575, t22587, t22589, t22591, t22592, t22595, t22596, t22602)
}
