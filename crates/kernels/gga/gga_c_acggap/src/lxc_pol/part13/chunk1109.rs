//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1109/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1109<F: Float>(t1181: F, t4930: F, t604: F, t7575: F, t4550: F, t1345: F, t1992: F, t30154: F, t7586: F, t1535: F, t4180: F, t7646: F) -> (F, F, F, F, F) {
    let t35219 = t7575 * t1181 * t604 * t4930;
    let t35223 = t7575 * t1181 * t604 * t4550;
    let t35225 = t1992 * t1345;
    let t35227 = t30154 * t7586 * t35225;
    let t35228 = F::new(0.14291339372689912324e-2) * t35227;
    let t35230 = t4180 * t7646 * t1535;
    (t35219, t35223, t35225, t35228, t35230)
}
