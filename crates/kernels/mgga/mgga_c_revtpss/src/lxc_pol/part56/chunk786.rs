//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 786/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk786<F: Float>(t25231: F, t2482: F, t27: F, t7043: F, t2677: F, t2712: F, t64: F, t2710: F, t826: F, t7036: F, t2487: F, t2689: F, t7030: F) -> (F, F, F, F, F, F, F, F) {
    let t25232 = F::new(0.27104001498285508387e-3) * t25231;
    let t25234 = t2482 * t7043 * t27;
    let t25235 = t25234 * t2677;
    let t25240 = t64 * t2712;
    let t25242 = t2710 * t25240 * t826;
    let t25243 = F::new(0.90357964994909313586e-5) * t25242;
    let t25245 = t2482 * t7036 * t27;
    let t25246 = t25245 * t2487;
    let t25253 = t2689 * t7030;
    (t25232, t25234, t25235, t25240, t25243, t25245, t25246, t25253)
}
