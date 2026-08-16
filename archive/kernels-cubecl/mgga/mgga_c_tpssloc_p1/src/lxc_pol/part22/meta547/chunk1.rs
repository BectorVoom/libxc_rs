//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2045/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2045<F: Float>(t3749: F, t40341: F, t59: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t12227: F, t9577: F, t116: F, t557: F) -> (F, F, F, F, F, F) {
    let t40343 = F::cast_from(0.99537037037037037035e-1_f64) * t40341 * t3749;
    let t40344 = t59 * t598;
    let t40347 = F::cast_from(0.11265432098765432099e0_f64) * t40344 * t535 * t795;
    let t40350 = F::cast_from(0.14979423868312757201e0_f64) * t39933 * t535 * t215;
    let t40351 = t9577 * t12227;
    let t40353 = t557 * t116;
    (t40343, t40344, t40347, t40350, t40351, t40353)
}
