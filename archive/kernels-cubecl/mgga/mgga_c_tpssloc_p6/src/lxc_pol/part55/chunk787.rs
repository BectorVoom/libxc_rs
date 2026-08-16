//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 787/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk787<F: Float>(t1653: F, t7363: F, t7362: F, t1716: F, t2148: F, t1755: F, t7376: F, t7375: F, t1751: F, t2147: F, t462: F, t1734: F, t2144: F) -> (F, F, F, F, F, F, F, F) {
    let t8066 = t7363 * t1653;
    let t8067 = t7362 * t8066;
    let t8070 = t1716 * t2148;
    let t8073 = t1755 * t7376;
    let t8074 = t7375 * t8073;
    let t8077 = t2147 * t1751;
    let t8078 = t462 * t8077;
    let t8082 = t2144 * t1734;
    (t8066, t8067, t8070, t8073, t8074, t8077, t8078, t8082)
}
