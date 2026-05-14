//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 699/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk699<F: Float>(t17771: F, t2493: F, t17761: F, t9916: F, t17766: F, t3910: F, t13313: F, t17749: F, t17753: F, t17757: F, t17736: F, t17776: F, t9896: F, t17740: F, t17744: F, t3917: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18324 = t2493 * t17771;
    let t18327 = t9916 * t17761;
    let t18330 = t3910 * t17766;
    let t18333 = t13313 * t17749;
    let t18336 = t3910 * t17753;
    let t18339 = t2493 * t17757;
    let t18342 = t2493 * t17736;
    let t18345 = t9896 * t17776;
    let t18348 = t2493 * t17740;
    let t18351 = t3917 * t17744;
    (t18324, t18327, t18330, t18333, t18336, t18339, t18342, t18345, t18348, t18351)
}
