//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 990/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk990<F: Float>(t1250: F, t19674: F, t6625: F, t7718: F, t1020: F, t19164: F, t7704: F, t2894: F, t356: F, t6556: F, t303: F, t6544: F, t7691: F, t5329: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28932 = t19674 * t1250;
    let t28935 = t7718 * t6625;
    let t28936 = t1020 * t28935;
    let t28938 = t7704 * t19164;
    let t28939 = t2894 * t28938;
    let t28944 = t356 * t6556;
    let t28945 = t303 * t28944;
    let t28947 = t7691 * t6544;
    let t28948 = t5329 * t28947;
    (t28932, t28935, t28936, t28938, t28939, t28944, t28945, t28947, t28948)
}
