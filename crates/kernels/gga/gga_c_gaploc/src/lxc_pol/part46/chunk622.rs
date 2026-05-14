//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 622/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk622<F: Float>(t12834: F, t2268: F, t3340: F, t894: F, t2765: F, t3137: F, t12821: F, t12823: F, t12824: F, t12825: F, t12828: F, t12829: F, t12832: F, t12833: F, t12818: F, t209: F) -> (F, F, F, F) {
    let t12836 = 0.28455006635676149599e-1 * t2268 * t12834;
    let t12837 = t894 * t3340;
    let t12838 = t2268 * t12837;
    let t12840 = t2765 * t3137;
    let t12842 = 0.85365019907028448797e-1 * t2268 * t12840;
    let t12843 = -0.23712505529730124666e-2 * t12821 - t12823 + t12824 + t12825 + t12828 + t12829 - t12832 - t12833 + t12836 + 0.56910013271352299198e-1 * t12838 - t12842;
    let t12844 = t12818 + t12843;
    let t12845 = t12844 * t209;
    (t12837, t12840, t12844, t12845)
}
