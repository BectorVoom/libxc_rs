//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 300/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk300<F: Float>(t321: F, t552: F, t333: F, t529: F, t941: F, t537: F, t809: F, t312: F, t50: F, t90: F, t814: F, t547: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t1551 = t552 * t321;
    let t1554 = t552 * t333;
    let t1562 = t941 * t529;
    let t1569 = t809 * t537;
    let t1570 = t1569 * t312;
    let t1573 = t90 * t50;
    let t1574 = t1573 * t814;
    let t1579 = t820 * t547;
    (t1551, t1554, t1562, t1569, t1570, t1573, t1574, t1579)
}
