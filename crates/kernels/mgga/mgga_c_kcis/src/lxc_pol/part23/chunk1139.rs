//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1139/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1139<F: Float>(t2104: F, t4457: F, t26391: F, t26399: F, t26401: F, t26409: F, t26655: F, t26520: F, t26558: F, t26517: F, t26417: F, t26632: F, t782: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t61664 = t2104 * t4457;
    let t91769 = F::new(18.0) * t26391;
    let t91772 = F::new(6.0) * t26399;
    let t91773 = F::new(12.0) * t26401;
    let t91776 = F::new(6.0) * t26409;
    let t91777 = F::new(3.0) * t26655;
    let t91778 = F::new(3.0) * t26520;
    let t91781 = F::new(3.0) * t26558;
    let t91785 = F::new(6.0) * t26517;
    let t91786 = F::new(6.0) * t26417;
    let t91789 = t26632 * t782;
    (t61664, t91769, t91772, t91773, t91776, t91777, t91778, t91781, t91785, t91786, t91789)
}
