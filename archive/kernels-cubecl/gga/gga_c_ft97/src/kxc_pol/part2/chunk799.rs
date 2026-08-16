//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 799/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk799<F: Float>(t12573: F, t446: F, t11003: F, t569: F, t3281: F, t11034: F, t2205: F, t2075: F, t3342: F, t28: F, t89: F, t1017: F, t1986: F, t7368: F) -> (F, F, F, F, F) {
    let t12574 = t446 * t12573;
    let t12576 = t569 * t11003;
    let t12577 = t3281 * t12576;
    let t12579 = t2205 * t11034;
    let t12580 = t446 * t12579;
    let t12582 = t3342 * t2075;
    let t12584 = t89 * t28 * t12582;
    let t12587 = t7368 * t1017 * t1986;
    (t12574, t12577, t12580, t12584, t12587)
}
