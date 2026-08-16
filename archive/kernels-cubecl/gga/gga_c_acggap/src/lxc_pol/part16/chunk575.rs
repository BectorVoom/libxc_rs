//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 575/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk575<F: Float>(t3451: F, t4925: F, t1541: F, t3372: F, t1005: F, t1352: F, t1588: F, t997: F, t3237: F, t542: F, t1581: F, t537: F) -> (F, F, F, F, F, F, F) {
    let t4926 = t3451 * t4925;
    let t4928 = t3372 * t1541;
    let t4946 = t1005 * t1352;
    let t4949 = F::cast_from(0.40015750243531754508e-2_f64) * t997 * t1588;
    let t4950 = t3237 * t542;
    let t4953 = F::cast_from(0.40015750243531754508e-2_f64) * t997 * t1581;
    let t4954 = t3237 * t537;
    (t4926, t4928, t4946, t4949, t4950, t4953, t4954)
}
