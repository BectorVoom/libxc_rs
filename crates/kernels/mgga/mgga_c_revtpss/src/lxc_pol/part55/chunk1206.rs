//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1206/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1206<F: Float>(t126148: F, t126166: F, t34068: F, t689: F, t119823: F, t119841: F, t121855: F, t121869: F, t126158: F, t126164: F, t2061: F, t27265: F, t27267: F, t7048: F, t7997: F, t8645: F, t8649: F, t8650: F) -> (F, F) {
    let t127662 = F::new(0.3718732920905101082e-4) * t126148;
    let t127675 = F::new(0.13223814266738539448e-3) * t126166;
    let t127676 = t34068 * t689;
    let t127677 = t119823 * t127676;
    let t127679 = t127662 - t119841 - t121855 + F::new(0.57119737665102352616e0) * t8649 * t8650 * t7997 * t7048 - F::new(0.8673628188205199462e0) * t8645 * t27267 + F::new(0.225875734067843736e-2) * t126158 + t121869 + F::new(0.57119737665102352616e0) * t8649 * t8650 * t2061 * t27265 - F::new(0.74374658418102021639e-4) * t126164 + t127675 - F::new(0.76169170176413987216e-1) * t127677;
    (t127676, t127679)
}
