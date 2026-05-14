//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 669/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk669<F: Float>(t123: F, t6540: F, t488: F, t1063: F, t1212: F, t1349: F, t1358: F, t2301: F, t2340: F, t3808: F, t4141: F, t6488: F, t6491: F, t6494: F, t6499: F, t6501: F, t6505: F, t6511: F, t6516: F, t6521: F, t6527: F, t6534: F, t6537: F, t889: F) -> (F, F) {
    let t6541 = t6540 * t123;
    let t6542 = t6541 * t488;
    let t6549 = -0.28455006635676149599e-1 * t1212 * t889 + 0.23712505529730124666e-2 * t6488 - 0.1138200265427045984e0 * t1063 * t6491 + 0.1707300398140568976e0 * t1063 * t6494 + 0.31616674039640166222e-2 * t6499 - 0.31616674039640166222e-2 * t6501 - 0.23712505529730124666e-2 * t6505 - 0.12646669615856066488e-1 * t1358 * t6511 + 0.18970004423784099732e-1 * t1358 * t6516 - 0.63233348079280332442e-2 * t1349 * t6521 - 0.23712505529730124666e-2 * t6527 - 0.31616674039640166222e-2 * t4141 * t2340 + 0.63233348079280332442e-2 * t3808 * t2340 + 0.23712505529730124666e-2 * t6534 + 0.63233348079280332442e-2 * t1358 * t6537 - 0.63233348079280332442e-2 * t1358 * t6542 + 0.31616674039640166222e-2 * t4141 * t2301 - 0.63233348079280332442e-2 * t3808 * t2301;
    (t6541, t6549)
}
