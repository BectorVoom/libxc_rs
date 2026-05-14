//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 684/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk684<F: Float>(t32300: F, t78: F, t371: F, t5544: F, t397: F, t7203: F, t7206: F, t356: F, t7204: F, t7202: F, t1691: F, t77: F, t8155: F, rho0: F) -> (F, F, F, F, F, F, F, F) {
    let t32301 = t32300 * t78;
    let t32304 = t371 * t5544;
    let t32307 = t7203 * t397;
    let t32308 = t32307 * t7206;
    let t32311 = t7204 * t356;
    let t32313 = 0.39129660776942540761e-2 * t7202 * t32311;
    let t32316 = t77 * t1691;
    let t32318 = 1.0 / t8155 / rho0;
    (t32301, t32304, t32307, t32308, t32311, t32313, t32316, t32318)
}
