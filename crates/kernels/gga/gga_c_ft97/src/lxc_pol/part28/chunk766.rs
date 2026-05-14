//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 766/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk766<F: Float>(t7166: F, t984: F, t28: F, t110: F, t1871: F, t34415: F, t7211: F, t979: F, t452: F, t488: F, t7274: F, t942: F, t1852: F, t34563: F, t83: F, t34566: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34619 = t7166 * t984;
    let t34620 = t28 * t34619;
    let t34624 = t1871 * t110 * t34415;
    let t34627 = t7211 * t979;
    let t34629 = t452 * t488 * t34627;
    let t34632 = t7274 * t942;
    let t34634 = t452 * t1852 * t34632;
    let t34637 = t83 * t34563;
    let t34640 = t83 * t34566;
    (t34619, t34620, t34624, t34627, t34629, t34632, t34634, t34637, t34640)
}
