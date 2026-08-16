//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1191/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1191(t10243: f64, t6305: f64, t26011: f64, t3822: f64, t894: f64, t1063: f64, t20395: f64, t2343: f64, t2787: f64, t3351: f64, t6338: f64, t10160: f64, t23927: f64) -> (f64, f64, f64, f64, f64) {
    let t32059 = 0.56910013271352299198e-1_f64 * t6305 * t10243;
    let t32062 = 0.56910013271352299198e-1_f64 * t3822 * t894 * t26011;
    let t32066 = 0.1138200265427045984e0_f64 * t1063 * t2343 * t2787 * t20395;
    let t32071 = t6338 * t3351;
    let t32072 = 0.11856252764865062333e-2_f64 * t32071;
    let t32073 = t23927 * t10160;
    (t32059, t32062, t32066, t32072, t32073)
}
