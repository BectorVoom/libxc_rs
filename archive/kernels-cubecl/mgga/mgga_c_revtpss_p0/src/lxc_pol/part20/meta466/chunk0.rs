//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1779/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1779<F: Float>(t10162: F, t9303: F, t3903: F, t9292: F, t1445: F, t2439: F, t9640: F, t3906: F, t3907: F, t39494: F, t1426: F, t4067: F, t786: F) -> (F, F, F, F, F) {
    let t47495 = t9303 * t10162;
    let t47497 = t9292 * t3903;
    let t47500 = t2439 * t9640 * t1445;
    let t47504 = F::cast_from(0.20561456923286030469e-1_f64) * t3906 * t3907 * t39494;
    let t47506 = t786 * t4067 * t1426;
    (t47495, t47497, t47500, t47504, t47506)
}
