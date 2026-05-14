//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 951/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk951<F: Float>(t2253: F, t4190: F, t12345: F, t1555: F, t7962: F, t4189: F, t4310: F, t4244: F, t573: F, t4248: F, t491: F) -> (F, F, F, F, F, F, F, F) {
    let t27503 = t2253 * t4190;
    let t27505 = 6.0 * t12345 * t27503;
    let t27506 = t7962 * t1555;
    let t27508 = 4.0 * t4189 * t27506;
    let t27509 = t2253 * t4310;
    let t27511 = 2.0 * t4189 * t27509;
    let t27512 = t4244 * t573;
    let t27514 = t4248 * t491;
    (t27503, t27505, t27506, t27508, t27509, t27511, t27512, t27514)
}
