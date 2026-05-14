//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1026/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1026<F: Float>(t4000: F, t820: F, t844: F, t5677: F, t13847: F, t13848: F, t1399: F, t9816: F, t2713: F, t3964: F, t5617: F, t5686: F, t9744: F, t221: F, t4019: F, t5659: F) -> (F, F, F, F, F) {
    let t13999 = t820 * t4000 * t844;
    let t14001 = 0.40015750243531754508e-2 * t13999 * t5677;
    let t14005 = t13847 * t13848 * t1399;
    let t14007 = 0.25410001404642664112e-4 * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    let t14024 = 7.0 / 24.0 * t9744 * t5686;
    let t14036 = t4019 * t221 * t5659;
    (t14001, t14007, t14013, t14024, t14036)
}
