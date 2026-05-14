//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1068/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1068<F: Float>(t21172: F, t2765: F, t3822: F, t1063: F, t20073: F, t2343: F, t2787: F, t25665: F, t894: F, t25841: F, t874: F, t2268: F, t10153: F, t10223: F, t1624: F, t31825: F, t31829: F, t31835: F, t31838: F, t31840: F, t31842: F, t31846: F, t3340: F, t535: F, t6305: F) -> (F, F) {
    let t31849 = 0.17073003981405689759e0 * t3822 * t2765 * t21172;
    let t31853 = 0.56910013271352299198e-1 * t1063 * t2343 * t2787 * t20073;
    let t31856 = 0.56910013271352299198e-1 * t1063 * t894 * t25665;
    let t31857 = t25841 * t874;
    let t31860 = 0.56910013271352299198e-1 * t2268 * t2343 * t31857;
    let t31861 = 0.28455006635676149599e-1 * t2268 * t1624 * t3340 + 0.56910013271352299198e-1 * t2268 * t535 * t10223 - t31825 + 0.1138200265427045984e0 * t6305 * t10153 + 0.1138200265427045984e0 * t2268 * t2343 * t31829 + t31835 + t31838 - t31840 - t31842 + t31846 - t31849 - t31853 - t31856 + t31860;
    (t31857, t31861)
}
