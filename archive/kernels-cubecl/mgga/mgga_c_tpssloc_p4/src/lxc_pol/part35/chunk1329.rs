//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1329/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1329<F: Float>(t29809: F, t85639: F, t1751: F, t8034: F, t29822: F, t29624: F, t491: F, t27381: F, t8009: F, t29585: F, t6686: F, t29614: F) -> (F, F, F, F, F, F, F) {
    let t103130 = t85639 * t29809;
    let t103143 = t8034 * t1751;
    let t103149 = t85639 * t29822;
    let t103175 = t29624 * t491;
    let t103188 = t8009 * t27381;
    let t103218 = t29585 * t6686;
    let t103226 = t29614 * t491;
    (t103130, t103143, t103149, t103175, t103188, t103218, t103226)
}
