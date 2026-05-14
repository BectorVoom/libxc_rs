//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 622/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk622<F: Float>(t262: F, t9708: F, t7198: F, t2350: F, t570: F, t7192: F, t1810: F, t2011: F, t291: F, t2010: F, t1661: F, t2415: F, t1665: F, t1948: F, t1986: F, t675: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9709 = t262 * t9708;
    let t9710 = t7198 * t9709;
    let t9711 = 0.81823984962736025184e-1 * t9710;
    let t9712 = t2350 * t570;
    let t9713 = t262 * t9712;
    let t9714 = t7192 * t9713;
    let t9715 = 0.27274661654245341728e-1 * t9714;
    let t9719 = t2011 * t1810;
    let t9720 = t9719 * t291;
    let t9721 = t2010 * t9720;
    let t9722 = 0.36021158228745895953e-3 * t9721;
    let t9723 = t2415 * t1661;
    let t9724 = t2010 * t9723;
    let t9725 = 0.72042316457491791906e-3 * t9724;
    let t9726 = t2415 * t1665;
    let t9727 = t2010 * t9726;
    let t9728 = 0.72042316457491791906e-3 * t9727;
    let t9731 = t1986 * t1948;
    let t9732 = t675 * t9731;
    (t9709, t9711, t9712, t9713, t9715, t9719, t9720, t9722, t9723, t9725, t9726, t9728, t9731, t9732)
}
