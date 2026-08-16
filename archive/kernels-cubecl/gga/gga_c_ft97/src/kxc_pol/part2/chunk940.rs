//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 940/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk940<F: Float>(t1578: F, t910: F, t4317: F, t5: F, t4376: F, t505: F, t2962: F, t992: F, t1274: F, t1934: F, t4394: F, t7742: F) -> (F, F, F, F, F, F) {
    let t14569 = t910 * t1578;
    let t14571 = t5 * t4317;
    let t14576 = t4376 * t505;
    let t14579 = t2962 * t992;
    let t14582 = t1274 * t1934;
    let t14593 = t4394 * t7742;
    (t14569, t14571, t14576, t14579, t14582, t14593)
}
