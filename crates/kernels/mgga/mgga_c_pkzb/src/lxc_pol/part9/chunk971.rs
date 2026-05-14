//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 971/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk971<F: Float>(t2464: F, t3282: F, t1259: F, t6362: F, t1306: F, t2461: F, t8147: F, t8185: F, t8187: F, t8191: F, t8194: F, t8197: F, t8201: F, t8204: F, t8208: F, t8216: F, t8218: F, t8221: F, t8237: F, t8295: F, t8298: F, t8302: F, t8305: F, t8307: F, t955: F) -> (F, F, F) {
    let t8568 = t3282 * t2464;
    let t8572 = t1259 * t6362;
    let t8576 = 2.0 * t1306 * t2461 * t8572 - 2.0 * t1306 * t8568 * t955 + t8147 - t8185 + t8187 - t8191 - t8194 - t8197 + t8201 + t8204 + t8208 + t8216 + t8218 + t8221 - t8237 - t8295 + t8298 - t8302 - t8305 + t8307;
    (t8568, t8572, t8576)
}
