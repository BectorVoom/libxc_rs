//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 959/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk959<F: Float>(t7839: F, t8518: F, t8522: F, t31699: F, t8526: F, t7637: F, t8506: F, t368: F, t4806: F, t1980: F, t7476: F, t2304: F, t7780: F) -> (F, F, F, F, F, F, F) {
    let t34035 = t7839 * t8518;
    let t34036 = F::new(0.21437009059034868486e-3) * t34035;
    let t34037 = t7839 * t8522;
    let t34038 = F::new(0.21437009059034868486e-3) * t34037;
    let t34039 = t31699 * t8526;
    let t34043 = t7637 * t8506;
    let t34050 = t368 * t4806;
    let t34052 = t1980 * t7476 * t34050;
    let t34053 = F::new(0.7145669686344956162e-3) * t34052;
    let t34054 = t7780 * t2304;
    (t34036, t34038, t34039, t34043, t34050, t34053, t34054)
}
