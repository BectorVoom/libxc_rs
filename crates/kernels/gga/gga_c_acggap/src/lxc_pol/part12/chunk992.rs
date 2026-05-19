//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 992/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk992<F: Float>(t2132: F, t30032: F, t309: F, t8336: F, t2241: F, t30005: F, t2217: F, t310: F, t464: F, t8357: F, t96: F, t104: F, t8355: F) -> (F, F, F, F, F, F) {
    let t33320 = F::cast_from(0.15612530738769359031e2_f64) * t30032 * t2132 * t8336 * t309;
    let t33321 = t30005 * t2241;
    let t33323 = t310 * t2217;
    let t33324 = t33323 * t464;
    let t33333 = t96 * t8357;
    let t33335 = t104 * t8355;
    (t33320, t33321, t33323, t33324, t33333, t33335)
}
