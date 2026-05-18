//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1007/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1007<F: Float>(t285: F, t6849: F, t2762: F, t328: F, t332: F, t2315: F, t286: F, t2801: F, t442: F, t8131: F, t2254: F, t8139: F) -> (F, F, F, F, F, F) {
    let t18679 = F::new(1.0) / t6849 / t285;
    let t18680 = F::new(1.0) / t2762 / t328 * t332 * t18679;
    let t18813 = t2315 * t286;
    let t18815 = t8131 * t2801 * t18813 * t442;
    let t18822 = t2254 * t286;
    let t18824 = t8139 * t18822 * t442;
    (t18679, t18680, t18813, t18815, t18822, t18824)
}
