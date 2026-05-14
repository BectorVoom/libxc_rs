//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1434/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1434<F: Float>(t114596: F, t114606: F, t114628: F, t114577: F, t114588: F, t114592: F, t114604: F, t114610: F, t114620: F, t114623: F, t114625: F, t114631: F, t32417: F, t32447: F, t33808: F, t33823: F, t33941: F, t9519: F) -> (F,) {
    let t115817 = 0.23214722222222222222e-2 * t114596;
    let t115819 = 0.15476481481481481481e-2 * t114606;
    let t115828 = 0.61905925925925925925e-2 * t114628;
    let t115830 = -0.11607361111111111111e-2 * t114577 - 0.10446625e-1 * t114588 - 0.18571777777777777777e-1 * t114592 + 0.34722222222222222222e-2 * t33941 * t32447 - t115817 + 0.51588271604938271604e-3 * t114604 + t115819 - 0.61905925925925925925e-2 * t114610 + 0.40208333333333333334e-2 * t32417 * t33823 + 0.10416666666666666667e-1 * t33808 * t9519 - 0.23214722222222222222e-2 * t114620 - 0.34822083333333333332e-2 * t114623 + 0.38691203703703703703e-3 * t114625 + t115828 + 0.11607361111111111111e-2 * t114631;
    (t115830,)
}
