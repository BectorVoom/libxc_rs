//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1280/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1280<F: Float>(t23405: F, t26815: F, t104446: F, t104450: F, t104453: F, t104463: F, t104465: F, t104467: F, t104469: F, t104471: F, t1647: F, t23403: F, t24080: F, t24104: F, t24148: F, t26817: F, t5772: F, t5775: F, t6580: F, t6587: F, t6589: F) -> (F,) {
    let t104474 = 2.0 / 3.0 * t23405 * t26815;
    let t104475 = -t104446 * t5775 / 9.0 + t104450 - t24148 * t6589 / 3.0 - 12.0 * t104453 - t26817 * t24104 / 9.0 - 2.0 / 9.0 * t5772 * t24080 * t6587 * t1647 + t6580 * t23403 + 4.0 * t104463 - 4.0 * t104465 - 2.0 * t104467 + 8.0 * t104469 + 4.0 * t104471 - t104474;
    (t104475,)
}
