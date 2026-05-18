//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 673/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk673<F: Float>(t2010: F, t9723: F, t1665: F, t2415: F, t1948: F, t1986: F, t675: F, t589: F, t597: F, t201: F, t1979: F, t1982: F) -> (F, F, F, F, F, F, F, F) {
    let t9724 = t2010 * t9723;
    let t9725 = F::new(0.72042316457491791906e-3) * t9724;
    let t9726 = t2415 * t1665;
    let t9727 = t2010 * t9726;
    let t9728 = F::new(0.72042316457491791906e-3) * t9727;
    let t9731 = t1986 * t1948;
    let t9732 = t675 * t9731;
    let t9733 = F::new(0.42564599893297839398e-5) * t9732;
    let t9734 = t589 * t597;
    let t9735 = t9734 * t201;
    let t9737 = t9735 * t1979 * t1982;
    (t9725, t9726, t9728, t9731, t9733, t9734, t9735, t9737)
}
