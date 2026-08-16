//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 709/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk709<F: Float>(t23471: F, t6740: F, t225: F, t343: F, t364: F, t3034: F, t371: F, t1930: F, t6741: F, t3030: F, t3127: F, t363: F) -> (F, F, F, F) {
    let t23472 = t6740 * t23471;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23508 = F::cast_from(1.0_f64) / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23510 = t23509 * t6741;
    let t23511 = t3030 * t3127;
    let t23512 = t23511 * t363;
    (t23472, t23479, t23510, t23512)
}
