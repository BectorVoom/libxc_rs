//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1230/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1230<F: Float>(t2189: F, t71840: F, t1020: F, t19781: F, t26760: F, t20671: F, t5329: F, t7773: F, t283: F, t6708: F, t7719: F, t1267: F, t28110: F, t5310: F, t6276: F) -> (F, F, F, F, F) {
    let t100034 = t71840 * t2189;
    let t100051 = t1020 * t26760 * t19781;
    let t100056 = t5329 * t7773 * t20671;
    let t100059 = t6708 * t283;
    let t100061 = t1020 * t100059 * t7719;
    let t100067 = t5310 * t28110 * t6276 * t1267;
    (t100034, t100051, t100056, t100061, t100067)
}
