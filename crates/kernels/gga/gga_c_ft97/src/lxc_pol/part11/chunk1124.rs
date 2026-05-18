//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1124/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1124<F: Float>(t191: F, t33828: F, t2682: F, t27: F, t89: F, t10389: F, t375: F, t2661: F, t41955: F, t10: F, t11175: F, t296: F) -> (F, F, F, F, F) {
    let t43524 = t191 * t33828;
    let t43525 = t2682 * t2682;
    let t43528 = t89 * t27 * t43524 * t43525;
    let t43531 = t89 * t375 * t10389;
    let t43534 = t89 * t41955 * t2661;
    let t43537 = t10 * t11175 * t296;
    (t43525, t43528, t43531, t43534, t43537)
}
