//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 302/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk302<F: Float>(t181: F, t1811: F, t1373: F, t1416: F, t1417: F, t1419: F, t618: F) -> (F, F, F, F, F, F) {
    let t1813 = F::cast_from(0.19751673498613801407e-1_f64) * t1811 * t181;
    let t1814 = F::cast_from(0.11696447245269292414e1_f64) * t1373;
    let t1815 = F::new(2.0) * t1416;
    let t1816 = F::new(8.0) * t1417;
    let t1817 = F::new(8.0) * t1419;
    let t1818 = t618 * t618;
    (t1813, t1814, t1815, t1816, t1817, t1818)
}
