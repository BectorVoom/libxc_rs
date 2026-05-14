//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 918/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk918<F: Float>(t1849: F, t3270: F, t220: F, t11524: F, t11528: F, t11532: F, t158: F, t16190: F, t16192: F, t16195: F, t16198: F, t16201: F, t16204: F, t16206: F, t16208: F, t16211: F, t16217: F, t16223: F, t16225: F, t16227: F, t16229: F) -> (F,) {
    let t16231 = t3270 * t1849;
    let t16232 = t16231 * t220;
    let t16234 = -0.31226666666666666666e-2 * t16190 + 0.7026e-2 * t158 * t16192 - 0.7026e-2 * t158 * t16195 + 0.1171e-2 * t158 * t16198 + 0.78066666666666666667e-3 * t158 * t16201 + 0.35222222222222222221e-2 * t16204 + 0.39210208333333333333e-4 * t16206 - 0.10929333333333333333e-1 * t16208 - 0.39814e-1 * t16211 + 0.10038333333333333333e-1 * t16217 + 0.77300125e-4 * t16223 + 0.23911438650126355246e-1 * t16225 - 0.31077233446777841256e-3 * t16227 + 0.11955719325063177623e0 * t16229 - 0.72513544709148296264e-3 * t16232 - t11524 + t11528 + t11532;
    (t16234,)
}
