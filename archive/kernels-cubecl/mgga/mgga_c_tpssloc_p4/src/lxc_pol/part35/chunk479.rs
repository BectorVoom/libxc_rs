//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 479/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk479<F: Float>(t59: F, t835: F, t154: F, t116: F, t206: F, t212: F, t2559: F, t222: F, t233: F, t813: F) -> (F, F, F, F, F, F, F, F) {
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t2588;
    let t2600 = t2559 * t154;
    let t2602 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2600 * t222;
    let t2627 = F::cast_from(1.0_f64) / t813 / t233;
    (t2585, t2586, t2587, t2588, t2590, t2600, t2602, t2627)
}
