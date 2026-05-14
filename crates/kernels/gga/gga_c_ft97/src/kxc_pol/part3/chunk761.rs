//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 761/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk761<F: Float>(t160: F, t17486: F, t16978: F, t17062: F, t17087: F, t17106: F, t17182: F, t17356: F, t17401: F, t17410: F, t17418: F, t17500: F, t17509: F, t184: F, t21: F, t3658: F) -> (F, F) {
    let t17510 = t17486 * t160;
    let t17522 = 2.0 * t17510 - 2.0 * t17410 - 4.0 * t17106 + 8.0 * t17401 - 4.0 * t17062 + 4.0 * t17418 - 12.0 * t17087 + 8.0 * t17182 - 2.0 * t17500 + 4.0 * t16978 - 2.0 * t17356;
    let t17523 = t17509 + t17522;
    let t17524 = t17523 * t184;
    let t17531 = t21 * t3658;
    (t17524, t17531)
}
