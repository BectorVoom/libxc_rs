//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 761/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk761<F: Float>(t1240: F, t225: F, t1251: F, t7300: F, t1190: F, t2144: F, t1193: F, t2127: F, t210: F, t2120: F) -> (F, F, F, F, F, F) {
    let t7301 = t225 * t1240;
    let t7302 = t7301 * t1251;
    let t7303 = t7300 * t7302;
    let t7306 = t1190 * t2144;
    let t7309 = t2127 * t1193 / F::cast_from(288.0_f64);
    let t7310 = t2120 * t210;
    (t7301, t7302, t7303, t7306, t7309, t7310)
}
