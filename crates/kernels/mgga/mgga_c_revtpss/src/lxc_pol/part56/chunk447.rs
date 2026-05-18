//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 447/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk447<F: Float>(t240: F, t823: F, t243: F, t836: F, t231: F, t2661: F, t596: F, t816: F, t813: F, t2482: F, t27: F, t849: F) -> (F, F, F, F, F, F, F) {
    let t2662 = t823 * t240;
    let t2663 = t243 * t836;
    let t2664 = t2663 * t231;
    let t2665 = t2662 * t2664;
    let t2666 = t2661 * t2665;
    let t2668 = t596 * t240;
    let t2670 = t2668 * t243 * t816;
    let t2672 = F::new(0.13552000749142754193e-3) * t813 * t2670;
    let t2674 = t2482 * t849 * t27;
    (t2662, t2664, t2666, t2668, t2670, t2672, t2674)
}
