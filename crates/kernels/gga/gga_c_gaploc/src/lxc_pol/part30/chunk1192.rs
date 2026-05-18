//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1192/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1192<F: Float>(t1063: F, t21042: F, t2765: F, t25955: F, t894: F, t20013: F, t2268: F, t2854: F, t6320: F, t2343: F, t6519: F, t7995: F) -> (F, F, F, F) {
    let t31945 = F::new(0.85365019907028448797e-1) * t1063 * t2765 * t21042;
    let t31948 = F::new(0.28455006635676149599e-1) * t1063 * t894 * t25955;
    let t31952 = F::new(0.17073003981405689759e0) * t2268 * t6320 * t2854 * t20013;
    let t31956 = F::new(0.1138200265427045984e0) * t1063 * t2343 * t7995 * t6519;
    (t31945, t31948, t31952, t31956)
}
