//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1185/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1185(t10178: f64, t6313: f64, t20539: f64, t2343: f64, t2787: f64, t3822: f64, t21172: f64, t2765: f64, t1063: f64, t20073: f64, t25665: f64, t894: f64) -> (f64, f64, f64, f64, f64) {
    let t31842 = 0.45528010617081839358e0_f64 * t6313 * t10178;
    let t31846 = 0.1138200265427045984e0_f64 * t3822 * t2343 * t2787 * t20539;
    let t31849 = 0.17073003981405689759e0_f64 * t3822 * t2765 * t21172;
    let t31853 = 0.56910013271352299198e-1_f64 * t1063 * t2343 * t2787 * t20073;
    let t31856 = 0.56910013271352299198e-1_f64 * t1063 * t894 * t25665;
    (t31842, t31846, t31849, t31853, t31856)
}
