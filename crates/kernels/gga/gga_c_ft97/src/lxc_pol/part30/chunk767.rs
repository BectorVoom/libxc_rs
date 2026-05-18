//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 767/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk767<F: Float>(t24737: F, t6166: F, t13885: F, t28128: F, t6175: F, t14127: F, t241: F, t258: F, t33531: F, t681: F, t7538: F, t89: F) -> (F, F, F, F, F, F) {
    let t33692 = t24737 * t6166;
    let t33693 = t13885 * t33692;
    let t33696 = t28128 * t6175;
    let t33697 = t14127 * t33696;
    let t33701 = t241 * t33531 * t258;
    let t33707 = t89 * t681 * t7538 / F::new(9.0);
    (t33692, t33693, t33696, t33697, t33701, t33707)
}
