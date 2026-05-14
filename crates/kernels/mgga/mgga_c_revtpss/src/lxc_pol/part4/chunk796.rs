//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 796/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk796<F: Float>(t4732: F, t4733: F, t981: F, t2848: F, t3037: F, t4571: F, t4576: F, t4581: F, t4585: F, t341: F) -> (F, F, F, F) {
    let t4734 = t4732 * t4733;
    let t4736 = 0.17315859105681463759e2 * t981 * t4734;
    let t4742 = t3037 + 0.27777777777777777778e-2 * t2848 + 0.27777777777777777778e-2 * t4571 - 0.55555555555555555555e-2 * t4576 + 0.16666666666666666667e-1 * t4581 - 0.83333333333333333333e-2 * t4585;
    let t4743 = t4742 * t341;
    (t4734, t4736, t4742, t4743)
}
