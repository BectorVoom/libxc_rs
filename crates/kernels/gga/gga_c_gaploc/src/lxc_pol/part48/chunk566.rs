//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 566/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk566<F: Float>(t11271: F, t494: F, t11167: F, t550: F, t1365: F, t197: F, t3516: F, t123: F, t4385: F, t1063: F, t11242: F, t11245: F, t11248: F, t11251: F, t11256: F, t11261: F, t11265: F, t11268: F, t1358: F, t2268: F) -> (F, F, F, F) {
    let t11272 = t11271 * t494;
    let t11275 = t550 * t11167;
    let t11276 = t1365 * t11275;
    let t11279 = t197 * t3516;
    let t11280 = t11279 * t123;
    let t11281 = t11280 * t4385;
    let t11284 = 0.1138200265427045984e0 * t2268 * t11242 + 0.85365019907028448797e-1 * t1063 * t11245 - 0.17073003981405689759e0 * t2268 * t11248 + 0.28455006635676149599e-1 * t2268 * t11251 + 0.56910013271352299198e-1 * t2268 * t11256 - 0.17073003981405689759e0 * t2268 * t11261 + 0.34146007962811379518e0 * t2268 * t11265 - 0.19918504644973304719e0 * t2268 * t11268 - 0.85365019907028448797e-1 * t2268 * t11272 + 0.31616674039640166221e-2 * t1358 * t11276 + 0.94850022118920498663e-2 * t1358 * t11281;
    (t11275, t11279, t11280, t11284)
}
