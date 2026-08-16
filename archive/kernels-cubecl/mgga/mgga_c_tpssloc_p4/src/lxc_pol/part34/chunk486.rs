//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 486/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk486<F: Float>(t1409: F, t751: F, t707: F, t75: F, t78: F, t1489: F, t2563: F, t131: F, t2570: F, t205: F, t1484: F, t213: F) -> (F, F, F, F, F, F, F) {
    let t4101 = t751 * t1409;
    let t4102 = t707 * t4101;
    let t4104 = t75 * t1409;
    let t4111 = t78 * t1409;
    let t4124 = t2563 * t1489;
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4128 = t213 * t1484;
    (t4101, t4102, t4104, t4111, t4124, t4127, t4128)
}
