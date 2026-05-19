//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 829/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk829<F: Float>(t550: F, t7980: F, t1365: F, t1063: F, t1358: F, t2268: F, t2766: F, t2784: F, t2789: F, t3808: F, t3822: F, t6305: F, t6313: F, t6451: F, t6457: F, t6460: F, t6468: F, t6472: F, t6488: F, t7952: F, t7958: F, t7964: F, t7968: F, t7971: F, t7975: F) -> (F, F) {
    let t7981 = t550 * t7980;
    let t7982 = t1365 * t7981;
    let t7991 = -F::cast_from(0.1707300398140568976e0_f64) * t6305 * t2766 + F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t7952 + F::cast_from(0.1138200265427045984e0_f64) * t6305 * t2789 - F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t7958 + F::cast_from(0.15176003539027279786e0_f64) * t6313 * t2789 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t7964 - F::cast_from(0.1138200265427045984e0_f64) * t1063 * t7968 + F::cast_from(0.1707300398140568976e0_f64) * t1063 * t7971 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t7975 + F::cast_from(0.63233348079280332442e-2_f64) * t3808 * t2784 + F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t7982 - F::cast_from(0.47425011059460249332e-2_f64) * t6451 + F::cast_from(0.47425011059460249332e-2_f64) * t6457 + F::cast_from(0.23712505529730124666e-2_f64) * t6460 - F::cast_from(0.23712505529730124666e-2_f64) * t6468 - F::cast_from(0.71137516589190373998e-2_f64) * t6472 + F::cast_from(0.47425011059460249332e-2_f64) * t6488;
    (t7981, t7991)
}
