//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 974/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk974<F: Float>(t28840: F, t7296: F, t72: F, t8103: F, t686: F, t7284: F, t1398: F, t543: F, t8085: F, t7301: F, t26265: F, t5722: F) -> (F, F, F, F, F) {
    let t28841 = t7296 * t28840;
    let t28844 = t8103 * t72;
    let t28845 = t28844 * t686;
    let t28846 = t7284 * t28845;
    let t28849 = t8085 * t1398 * t543;
    let t28850 = t7301 * t28849;
    let t28853 = t26265 * t5722;
    (t28841, t28845, t28846, t28850, t28853)
}
