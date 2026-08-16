//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1371/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1371(t1063: f64, t11983: f64, t1306: f64, t1624: f64, t2268: f64, t30091: f64, t30094: f64, t30096: f64, t30098: f64, t30103: f64, t31900: f64, t31906: f64, t31909: f64, t31912: f64, t31915: f64, t31919: f64, t3691: f64, t3701: f64, t6305: f64) -> f64 {
    let t38382 = -t31900 - t31906 - t31909 - 0.28455006635676149599e-1_f64 * t1063 * t3701 * t1306 + 0.28455006635676149599e-1_f64 * t2268 * t1624 * t3691 + 0.1138200265427045984e0_f64 * t6305 * t11983 + t31912 + t31915 + t30091 + t30094 + t30096 - t31919 + t30098 + t30103;
    t38382
}
