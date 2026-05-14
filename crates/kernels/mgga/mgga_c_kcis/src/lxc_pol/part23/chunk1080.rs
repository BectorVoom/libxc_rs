//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1080/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1080<F: Float>(t17509: F, t94785: F, t28589: F, t4262: F, t17490: F, t27520: F, t27529: F, t28610: F, t27526: F, t1928: F, t4248: F, t7949: F, t17352: F, t4123: F, t27532: F, t28640: F) -> (F, F, F, F, F, F, F, F) {
    let t97717 = t94785 * t17509;
    let t97719 = t28589 * t4262;
    let t97721 = t27520 * t17490;
    let t97723 = t28610 * t27529;
    let t97725 = t28589 * t27526;
    let t97727 = t4248 * t1928;
    let t97728 = t97727 * t7949;
    let t97730 = t4123 * t17352;
    let t97732 = t28640 * t27532;
    (t97717, t97719, t97721, t97723, t97725, t97728, t97730, t97732)
}
