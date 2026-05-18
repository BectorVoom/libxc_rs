//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 855/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk855<F: Float>(t27634: F, t7160: F, t1078: F, t11239: F, t1035: F, t1983: F, t1668: F, t1976: F, t3153: F, t4998: F, t1043: F, t1089: F, t7828: F) -> (F, F, F, F, F, F, F) {
    let t27635 = t7160 * t27634;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    let t27641 = t1976 * t1668;
    let t27642 = t27641 * t3153;
    let t27643 = t27642 * t4998;
    let t27647 = t7828 * t1043 * t1089;
    (t27635, t27638, t27640, t27641, t27642, t27643, t27647)
}
