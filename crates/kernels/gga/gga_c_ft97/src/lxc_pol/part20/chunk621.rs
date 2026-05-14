//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 621/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk621<F: Float>(t13757: F, t9770: F, t446: F, t13728: F, t13732: F, t13736: F, t13740: F, t13743: F, t13747: F, t13750: F, t13754: F, t9765: F, t9768: F, t2514: F, t3902: F, t91: F) -> (F, F, F) {
    let t13758 = t9770 * t13757;
    let t13759 = t446 * t13758;
    let t13761 = 4.0 / 3.0 * t13728 - 22.0 / 27.0 * t13732 + 2.0 / 9.0 * t13736 - t13740 + 2.0 / 3.0 * t13743 - t13747 - t13750 / 3.0 + t13754 - 2.0 / 27.0 * t9768 - 2.0 / 27.0 * t9765 - 4.0 / 9.0 * t13759;
    let t13764 = t91 * t3902 * t2514;
    (t13759, t13761, t13764)
}
