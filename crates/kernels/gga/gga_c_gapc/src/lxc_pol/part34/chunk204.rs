//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 204/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk204<F: Float>(t408: F, t411: F, t414: F, t419: F, t88: F, t257: F, t260: F, t266: F, t738: F, t748: F, t751: F) -> (F, F) {
    let t758 = F::new(0.77371026992393176896e-2) * t88 - F::new(0.2499945e-2) * t408 + F::new(0.604634375e-3) * t411 - F::new(0.20417003743104289064e-4) * t414 + F::new(0.20205871875e-5) * t419;
    let t760 = -F::new(0.10636476373080147432e-2) * t88 * t257 - F::new(0.21272952746160294864e-2) * t738 * t748 - t751 * t266 - t260 * t758;
    (t758, t760)
}
