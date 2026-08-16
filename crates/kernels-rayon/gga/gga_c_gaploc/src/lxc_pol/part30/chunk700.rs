//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 700/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk700(t123: f64, t6540: f64, t488: f64, t1063: f64, t1212: f64, t1349: f64, t1358: f64, t2301: f64, t2340: f64, t3808: f64, t4141: f64, t6488: f64, t6491: f64, t6494: f64, t6499: f64, t6501: f64, t6505: f64, t6511: f64, t6516: f64, t6521: f64, t6527: f64, t6534: f64, t6537: f64, t889: f64) -> (f64, f64) {
    let t6541 = t6540 * t123;
    let t6542 = t6541 * t488;
    let t6549 = -0.28455006635676149599e-1_f64 * t1212 * t889 + 0.23712505529730124666e-2_f64 * t6488 - 0.1138200265427045984e0_f64 * t1063 * t6491 + 0.1707300398140568976e0_f64 * t1063 * t6494 + 0.31616674039640166222e-2_f64 * t6499 - 0.31616674039640166222e-2_f64 * t6501 - 0.23712505529730124666e-2_f64 * t6505 - 0.12646669615856066488e-1_f64 * t1358 * t6511 + 0.18970004423784099732e-1_f64 * t1358 * t6516 - 0.63233348079280332442e-2_f64 * t1349 * t6521 - 0.23712505529730124666e-2_f64 * t6527 - 0.31616674039640166222e-2_f64 * t4141 * t2340 + 0.63233348079280332442e-2_f64 * t3808 * t2340 + 0.23712505529730124666e-2_f64 * t6534 + 0.63233348079280332442e-2_f64 * t1358 * t6537 - 0.63233348079280332442e-2_f64 * t1358 * t6542 + 0.31616674039640166222e-2_f64 * t4141 * t2301 - 0.63233348079280332442e-2_f64 * t3808 * t2301;
    (t6541, t6549)
}
