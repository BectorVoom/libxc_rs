//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1044/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1044<F: Float>(t30347: F, t30811: F, t4273: F, t2068: F, t7727: F, t8480: F, t129: F, t507: F, t7585: F, t7587: F, t30546: F, t8477: F) -> (F, F, F, F, F) {
    let t34339 = F::new(0.42874018118069736972e-3) * t30347;
    let t34340 = t30811 * t4273;
    let t34341 = F::new(0.68598428988911579156e-2) * t34340;
    let t34343 = t2068 * t8480 * t7727;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34348 = F::new(0.14291339372689912324e-3) * t34347;
    let t34349 = t30546 * t8477;
    (t34339, t34341, t34343, t34348, t34349)
}
