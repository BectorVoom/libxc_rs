//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2929/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929<F: Float>(t14667: F, t17198: F, t17202: F, t3209: F, t4700: F, t60398: F, t60400: F, t60429: F, t60434: F, t60568: F, t60570: F, t60941: F, t60946: F, t60953: F, t60955: F, t60958: F, t60961: F) -> F {
    let t60962 = F::cast_from(8.0_f64) * t14667 * t4700 * t60941 + F::cast_from(2.0_f64) * t17198 * t3209 * t4700 - t17202 * t3209 * t4700 - t60398 + t60400 + t60429 + t60434 + t60568 + t60570 - t60946 - t60953 - t60955 - t60958 + t60961;
    t60962
}
