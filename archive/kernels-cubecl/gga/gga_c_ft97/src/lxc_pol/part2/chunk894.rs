//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 894/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk894<F: Float>(t10079: F, t13853: F, t2567: F, t668: F, t2569: F, t992: F, t2606: F, t2349: F) -> (F, F, F) {
    let t13854 = t10079 * t13853;
    let t13857 = t2567 * t668;
    let t13858 = t992 * t2569;
    let t13859 = t13857 * t13858;
    let t13860 = t2606 * t13859;
    let t13863 = t992 * t2349;
    (t13854, t13860, t13863)
}
