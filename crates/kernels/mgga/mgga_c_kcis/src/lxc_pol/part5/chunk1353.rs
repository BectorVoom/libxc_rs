//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1353/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1353<F: Float>(t1555: F, t7271: F, t12345: F, t2069: F, t6048: F, t4189: F, t4184: F, t7397: F, t1529: F, t7386: F, t1543: F, t7329: F) -> (F, F, F, F, F, F) {
    let t22310 = t7271 * t1555;
    let t22312 = F::cast_from(6.0_f64) * t12345 * t22310;
    let t22313 = t2069 * t6048;
    let t22315 = F::cast_from(4.0_f64) * t4189 * t22313;
    let t22316 = t4184 * t7397;
    let t22317 = t7397 * t1555;
    let t22319 = F::cast_from(2.0_f64) * t4189 * t22317;
    let t22320 = t1529 * t7386;
    let t22322 = t1543 * t7329;
    (t22312, t22315, t22316, t22319, t22320, t22322)
}
