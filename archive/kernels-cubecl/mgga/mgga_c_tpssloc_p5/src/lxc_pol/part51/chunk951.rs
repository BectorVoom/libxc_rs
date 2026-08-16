//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 951/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk951<F: Float>(t225: F, t6625: F, t6576: F, t2752: F, t6665: F, t10143: F, t1914: F, t221: F, t2987: F, t1926: F) -> (F, F, F, F, F) {
    let t23278 = t6625 * t225;
    let t23281 = t6576 * t225;
    let t23290 = t6665 * t2752;
    let t23295 = t1914 * t10143;
    let t23326 = t221 * t2987;
    let t23327 = t1926 * t23326;
    (t23278, t23281, t23290, t23295, t23327)
}
