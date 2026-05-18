//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 895/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk895<F: Float>(t39832: F, t7478: F, t1175: F, t1971: F, t515: F, t570: F, t8517: F, t34884: F, t9046: F, t2289: F, t34881: F, t16501: F, t7363: F) -> (F, F, F, F, F) {
    let t39833 = t39832 * t7478;
    let t39838 = t8517 * t1971 * t515 * t570 * t1175;
    let t39840 = t34884 * t9046;
    let t39842 = t34881 * t2289;
    let t39850 = t7363 * t16501;
    (t39833, t39838, t39840, t39842, t39850)
}
