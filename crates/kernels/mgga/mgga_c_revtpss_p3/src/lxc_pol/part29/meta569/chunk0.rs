//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1916/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1916<F: Float>(t1468: F, t2832: F, t2408: F, t25207: F, t61182: F, t2430: F, t1583: F, t2257: F, t2394: F, t11064: F, t605: F, t27384: F) -> (F, F, F, F, F, F, F, F) {
    let t98736 = t1468 * t2832;
    let t98740 = t1468 * t2408;
    let t98743 = t25207 * t61182;
    let t98751 = t1468 * t2430;
    let t98755 = t2257 * t1583;
    let t98759 = t1583 * t2394;
    let t98760 = t25207 * t98759;
    let t98763 = t11064 * t605;
    let t98764 = t98763 * t27384;
    (t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98764)
}
