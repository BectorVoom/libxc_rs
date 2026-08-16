//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1201/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1201(t10167: f64, t29874: f64, t10269: f64, t4141: f64, t10196: f64, t3833: f64, t10243: f64, t6305: f64, t26011: f64, t3822: f64, t894: f64, t1063: f64, t20395: f64, t2343: f64, t2787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32052 = t29874 * t10167;
    let t32053 = 0.71137516589190373998e-2_f64 * t32052;
    let t32055 = 0.63233348079280332441e-2_f64 * t4141 * t10269;
    let t32057 = 0.56910013271352299198e-1_f64 * t3833 * t10196;
    let t32059 = 0.56910013271352299198e-1_f64 * t6305 * t10243;
    let t32062 = 0.56910013271352299198e-1_f64 * t3822 * t894 * t26011;
    let t32066 = 0.1138200265427045984e0_f64 * t1063 * t2343 * t2787 * t20395;
    (t32053, t32055, t32057, t32059, t32062, t32066)
}
