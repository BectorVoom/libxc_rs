//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 708/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk708<F: Float>(t4669: F, t954: F, t1621: F, t2970: F, t953: F, t2848: F, t2974: F, t4571: F, t4576: F, t4581: F, t4585: F, t324: F, t1626: F, t964: F, t1634: F, t972: F) -> (F, F, F, F, F, F, F) {
    let t4670 = t4669 * t954;
    let t4673 = t1621 * t2970;
    let t4674 = t4673 * t953;
    let t4682 = t2974 + 0.30902777777777777778e-2 * t2848 + 0.30902777777777777778e-2 * t4571 - 0.61805555555555555555e-2 * t4576 + 0.18541666666666666667e-1 * t4581 - 0.92708333333333333333e-2 * t4585;
    let t4683 = t4682 * t324;
    let t4685 = t1626 * t964;
    let t4690 = t1634 * t972;
    (t4670, t4673, t4674, t4682, t4683, t4685, t4690)
}
