//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1445/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445<F: Float>(t40834: F, t61837: F, t854: F, t10886: F, t18608: F, t808: F, t18352: F, t2710: F, t2713: F, t10722: F, t6030: F, t18419: F, t9775: F) -> (F, F, F, F, F) {
    let t61839 = t40834 * t854 * t61837;
    let t61877 = t10886 * t808 * t18608;
    let t61888 = t2710 * t2713 * t18352;
    let t61890 = t10722 * t6030;
    let t61892 = t9775 * t18419;
    (t61839, t61877, t61888, t61890, t61892)
}
