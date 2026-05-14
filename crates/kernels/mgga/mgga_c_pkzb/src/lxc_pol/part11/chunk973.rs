//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 973/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk973<F: Float>(t4865: F, t4871: F, t1531: F, t466: F, t5342: F, t5089: F, t1504: F, t1507: F, t4911: F, t555: F, t1517: F, t1527: F, t4999: F, t5002: F, t1511: F, t1502: F) -> (F, F, F, F, F, F, F, F) {
    let t16532 = t4871 * t4865;
    let t16536 = 0.21687162600603479684e-1 * t1531 * t466 * t5342;
    let t16539 = 0.38527786510141256862e1 * t1531 * t466 * t5089;
    let t16540 = t1504 * t1504;
    let t16544 = 0.6233709278045326953e3 * t555 * t4911 * t16540 * t1507;
    let t16548 = 0.3103560775156404018e4 * t4999 * t1517 * t5002 * t1527;
    let t16554 = t1511 * t5342;
    let t16556 = t1502 * t1502;
    (t16532, t16536, t16539, t16540, t16544, t16548, t16554, t16556)
}
