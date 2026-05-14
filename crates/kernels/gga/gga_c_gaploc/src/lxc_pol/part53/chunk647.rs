//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 647/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk647<F: Float>(t12769: F, t12799: F, t12802: F, t12805: F, t12809: F, t12812: F, t13726: F, t13730: F, t13733: F, t13741: F, t13753: F, t12823: F, t12824: F, t12825: F, t12828: F, t12829: F, t12832: F, t12833: F, t12836: F, t12842: F, t13736: F, t13758: F) -> (F, F) {
    let t14450 = t12769 - 0.23712505529730124666e-2 * t13741 + 0.1138200265427045984e0 * t13730 + t12799 - t12802 + t12805 + 0.23712505529730124666e-2 * t13726 - 0.17073003981405689759e0 * t13733 - t12809 - t13753 + t12812;
    let t14452 = 0.56910013271352299198e-1 * t13736 + t13758 - t12823 + t12824 + t12825 + t12828 + t12829 - t12832 - t12833 + t12836 - t12842;
    (t14450, t14452)
}
