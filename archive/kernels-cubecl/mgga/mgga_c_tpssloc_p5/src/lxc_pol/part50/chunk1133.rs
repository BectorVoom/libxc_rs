//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1133/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1133<F: Float>(t30624: F, t81591: F, t30635: F, t6579: F, t23185: F, t30634: F, t82074: F, t6662: F, t857: F, t30667: F, t6547: F, t6562: F, t82133: F, t8335: F) -> (F, F, F, F, F, F) {
    let t112680 = t81591 * t30624;
    let t112686 = t6579 * t30635;
    let t112702 = t23185 * t82074 * t30634;
    let t112719 = t857 * t6662;
    let t112726 = t6547 * t30667;
    let t112741 = t6562 * t82133 * t8335;
    (t112680, t112686, t112702, t112719, t112726, t112741)
}
