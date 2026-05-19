//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1375/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1375<F: Float>(t12000: F, t203: F, t447: F, t12068: F, t1328: F, t475: F, t1063: F, t1064: F, t11977: F, t11981: F, t11991: F, t11994: F, t12116: F, t1595: F, t2268: F, t2343: F, t30152: F, t30169: F, t30171: F, t32012: F, t3691: F, t3818: F, t3833: F, t4324: F, t448: F, t4807: F, t6305: F, t6313: F, t6320: F) -> (F, F, F, F, F) {
    let t38413 = t203 * t12000;
    let t38414 = t38413 * t447;
    let t38429 = t12068 * t1328;
    let t38436 = t38413 * t475;
    let t38444 = -t30152 - t32012 + F::cast_from(0.1707300398140568976e0_f64) * t1063 * t11977 * t4807 + F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t1064 * t38414 - F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t11991 + F::cast_from(0.56910013271352299198e-1_f64) * t6305 * t11994 - t30169 - F::cast_from(0.1138200265427045984e0_f64) * t1063 * t2343 * t11981 * t4324 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t12116 * t448 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6320 * t38429 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t1595 * t3691 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t1064 * t38436 - F::cast_from(0.7588001769513639893e-1_f64) * t3818 * t11991 + F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t11994 + t30171;
    (t38413, t38414, t38429, t38436, t38444)
}
