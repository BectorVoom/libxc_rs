//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1027/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1027<F: Float>(t36131: F, t1988: F, t8502: F, t7799: F, t8506: F, t2290: F, t7780: F, t1423: F, t7746: F, t1507: F, t2020: F, t30120: F, t8793: F) -> (F, F, F, F, F, F, F) {
    let t36132 = F::new(0.42874018118069736972e-3) * t36131;
    let t36133 = t1988 * t8502;
    let t36134 = F::new(0.42874018118069736972e-3) * t36133;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    let t36139 = t7746 * t1423;
    let t36151 = t2020 * t1507;
    let t36152 = F::new(7.0) / F::new(144.0) * t36151;
    let t36156 = t30120 * t8793;
    (t36132, t36134, t36135, t36137, t36139, t36152, t36156)
}
