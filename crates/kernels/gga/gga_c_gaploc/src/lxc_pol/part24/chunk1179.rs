//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1179/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1179<F: Float>(t2268: F, t2765: F, t6474: F, t23741: F, t3327: F, t10113: F, t6305: F, t23726: F, t2293: F, t7995: F, t2343: F, t10151: F, t1328: F) -> (F, F, F, F, F, F, F) {
    let t31704 = F::new(0.85365019907028448797e-1) * t2268 * t2765 * t6474;
    let t31706 = F::new(0.28455006635676149599e-1) * t23741 * t3327;
    let t31708 = F::new(0.56910013271352299198e-1) * t6305 * t10113;
    let t31710 = F::new(0.7588001769513639893e-1) * t23726 * t3327;
    let t31711 = t7995 * t2293;
    let t31714 = F::new(0.1138200265427045984e0) * t2268 * t2343 * t31711;
    let t31715 = t10151 * t1328;
    (t31704, t31706, t31708, t31710, t31711, t31714, t31715)
}
