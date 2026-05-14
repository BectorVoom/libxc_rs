//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 781/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk781<F: Float>(t1340: F, t7937: F, t1529: F, t993: F, t2268: F, t2738: F, t2741: F, t2766: F, t3818: F, t3833: F, t6293: F, t6297: F, t6300: F, t6302: F, t6305: F, t6313: F, t6328: F, t6334: F, t6339: F, t7916: F, t7919: F, t7931: F, t7934: F) -> (F,) {
    let t7938 = t7937 * t1340;
    let t7941 = t1529 * t993;
    let t7948 = 0.47425011059460249332e-2 * t6293 - 0.23712505529730124666e-2 * t6297 + 0.23712505529730124666e-2 * t6300 + 0.63233348079280332442e-2 * t6302 + 0.73772239425827054516e-2 * t6328 + 0.28455006635676149599e-1 * t2268 * t7916 + 0.56910013271352299198e-1 * t2268 * t7919 + 0.7588001769513639893e-1 * t6313 * t2741 - 0.56910013271352299198e-1 * t3833 * t2738 + 0.56910013271352299198e-1 * t6305 * t2741 - 0.7588001769513639893e-1 * t3818 * t2738 - 0.1707300398140568976e0 * t2268 * t7931 + 0.28455006635676149599e-1 * t2268 * t7934 + 0.34146007962811379518e0 * t2268 * t7938 - 0.19918504644973304719e0 * t2268 * t7941 - 0.63233348079280332442e-2 * t6334 + 0.23712505529730124666e-2 * t6339 - 0.22764005308540919679e0 * t6313 * t2766;
    (t7948,)
}
