//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1032/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1032<F: Float>(t1988: F, t8502: F, t7799: F, t8506: F, t2290: F, t7780: F, t1423: F, t7746: F, t31752: F, t1507: F, t2020: F, t30120: F, t8793: F) -> (F, F, F, F, F, F, F) {
    let t36133 = t1988 * t8502;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    let t36139 = t7746 * t1423;
    let t36141 = F::new(0.26416397523267487738e-1) * t31752;
    let t36151 = t2020 * t1507;
    let t36156 = t30120 * t8793;
    (t36133, t36135, t36137, t36139, t36141, t36151, t36156)
}
