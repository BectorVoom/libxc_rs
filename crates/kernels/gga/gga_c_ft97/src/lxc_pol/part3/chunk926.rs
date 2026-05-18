//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 926/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk926<F: Float>(t17727: F, t3917: F, t17732: F, t3910: F, t17771: F, t2493: F, t17761: F, t9916: F, t17766: F, t13313: F, t17749: F, t17753: F) -> (F, F, F, F, F, F, F) {
    let t18318 = t3917 * t17727;
    let t18321 = t3910 * t17732;
    let t18324 = t2493 * t17771;
    let t18327 = t9916 * t17761;
    let t18330 = t3910 * t17766;
    let t18333 = t13313 * t17749;
    let t18336 = t3910 * t17753;
    (t18318, t18321, t18324, t18327, t18330, t18333, t18336)
}
