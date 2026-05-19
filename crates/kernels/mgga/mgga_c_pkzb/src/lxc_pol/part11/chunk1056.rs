//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1056/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1056<F: Float>(t1504: F, t1507: F, t4911: F, t555: F, t1517: F, t1527: F, t4999: F, t5002: F, t1511: F, t5342: F, t1502: F, t1506: F) -> (F, F, F, F, F, F) {
    let t16540 = t1504 * t1504;
    let t16544 = F::cast_from(0.6233709278045326953e3_f64) * t555 * t4911 * t16540 * t1507;
    let t16548 = F::cast_from(0.3103560775156404018e4_f64) * t4999 * t1517 * t5002 * t1527;
    let t16554 = t1511 * t5342;
    let t16556 = t1502 * t1502;
    let t16557 = F::new(1.0) / t16556;
    let t16559 = t1506 * t1506;
    (t16540, t16544, t16548, t16554, t16557, t16559)
}
