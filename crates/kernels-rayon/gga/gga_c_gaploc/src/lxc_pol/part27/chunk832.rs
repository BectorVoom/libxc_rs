//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 832/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk832(t550: f64, t7980: f64, t1365: f64, t1063: f64, t1358: f64, t2268: f64, t2766: f64, t2784: f64, t2789: f64, t3808: f64, t3822: f64, t6305: f64, t6313: f64, t6451: f64, t6457: f64, t6460: f64, t6468: f64, t6472: f64, t6488: f64, t7952: f64, t7958: f64, t7964: f64, t7968: f64, t7971: f64, t7975: f64) -> (f64, f64) {
    let t7981 = t550 * t7980;
    let t7982 = t1365 * t7981;
    let t7991 = -0.1707300398140568976e0_f64 * t6305 * t2766 + 0.56910013271352299198e-1_f64 * t3822 * t7952 + 0.1138200265427045984e0_f64 * t6305 * t2789 - 0.56910013271352299198e-1_f64 * t3822 * t7958 + 0.15176003539027279786e0_f64 * t6313 * t2789 - 0.17073003981405689759e0_f64 * t2268 * t7964 - 0.1138200265427045984e0_f64 * t1063 * t7968 + 0.1707300398140568976e0_f64 * t1063 * t7971 - 0.56910013271352299198e-1_f64 * t1063 * t7975 + 0.63233348079280332442e-2_f64 * t3808 * t2784 + 0.63233348079280332442e-2_f64 * t1358 * t7982 - 0.47425011059460249332e-2_f64 * t6451 + 0.47425011059460249332e-2_f64 * t6457 + 0.23712505529730124666e-2_f64 * t6460 - 0.23712505529730124666e-2_f64 * t6468 - 0.71137516589190373998e-2_f64 * t6472 + 0.47425011059460249332e-2_f64 * t6488;
    (t7981, t7991)
}
