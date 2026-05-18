//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 939/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk939<F: Float>(t1835: F, t1979: F, t1982: F, t201: F, t457: F, t2191: F, t9932: F, t9935: F, t1986: F, t6592: F, t675: F, t1743: F, t352: F) -> (F, F, F, F, F) {
    let t45608 = t1835 * t457 * t201 * t1979 * t1982;
    let t45610 = t2191 * t9932;
    let t45614 = t2191 * t9935;
    let t45617 = t675 * t1986 * t6592;
    let t45622 = t1743 * t352;
    (t45608, t45610, t45614, t45617, t45622)
}
