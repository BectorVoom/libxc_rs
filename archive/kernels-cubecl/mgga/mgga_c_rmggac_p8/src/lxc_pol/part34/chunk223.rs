//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 223/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk223<F: Float>(t1579: F, t316: F, t101: F, t50: F, t814: F, t1570: F, t1574: F, t309: F, t317: F, t538: F, t544: F, t832: F, t87: F, t98: F) -> F {
    let t1580 = t1579 * t316;
    let t1583 = t101 * t50;
    let t1584 = t1583 * t814;
    let t1587 = -F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t309 * t538 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t87 * t1570 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t87 * t1574 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t544 * t317 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t98 * t1580 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t98 * t1584 - t832;
    t1587
}
