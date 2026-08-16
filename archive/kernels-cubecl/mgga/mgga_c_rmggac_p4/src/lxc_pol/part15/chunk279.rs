//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 279/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk279<F: Float>(t537: F, t809: F, t312: F, t50: F, t90: F, t814: F, t547: F, t820: F, t316: F, t101: F, t309: F, t317: F, t538: F, t544: F, t832: F, t87: F, t98: F) -> (F, F, F) {
    let t1569 = t809 * t537;
    let t1570 = t1569 * t312;
    let t1573 = t90 * t50;
    let t1574 = t1573 * t814;
    let t1579 = t820 * t547;
    let t1580 = t1579 * t316;
    let t1583 = t101 * t50;
    let t1584 = t1583 * t814;
    let t1587 = -F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t309 * t538 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t87 * t1570 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t87 * t1574 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t544 * t317 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t98 * t1580 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t98 * t1584 - t832;
    (t1580, t1584, t1587)
}
