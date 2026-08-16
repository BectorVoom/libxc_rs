//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1373/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1373(t12000: f64, t203: f64, t447: f64, t12068: f64, t1328: f64, t475: f64, t1063: f64, t1064: f64, t11977: f64, t11981: f64, t11991: f64, t11994: f64, t12116: f64, t1595: f64, t2268: f64, t2343: f64, t30152: f64, t30169: f64, t30171: f64, t32012: f64, t3691: f64, t3818: f64, t3833: f64, t4324: f64, t448: f64, t4807: f64, t6305: f64, t6313: f64, t6320: f64) -> (f64, f64, f64, f64, f64) {
    let t38413 = t203 * t12000;
    let t38414 = t38413 * t447;
    let t38429 = t12068 * t1328;
    let t38436 = t38413 * t475;
    let t38444 = -t30152 - t32012 + 0.1707300398140568976e0_f64 * t1063 * t11977 * t4807 + 0.56910013271352299198e-1_f64 * t1063 * t1064 * t38414 - 0.56910013271352299198e-1_f64 * t3833 * t11991 + 0.56910013271352299198e-1_f64 * t6305 * t11994 - t30169 - 0.1138200265427045984e0_f64 * t1063 * t2343 * t11981 * t4324 - 0.56910013271352299198e-1_f64 * t1063 * t12116 * t448 - 0.17073003981405689759e0_f64 * t2268 * t6320 * t38429 + 0.28455006635676149599e-1_f64 * t2268 * t1595 * t3691 - 0.17073003981405689759e0_f64 * t2268 * t1064 * t38436 - 0.7588001769513639893e-1_f64 * t3818 * t11991 + 0.7588001769513639893e-1_f64 * t6313 * t11994 + t30171;
    (t38413, t38414, t38429, t38436, t38444)
}
