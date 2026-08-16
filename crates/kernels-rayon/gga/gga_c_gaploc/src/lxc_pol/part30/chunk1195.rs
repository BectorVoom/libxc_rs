//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1195/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1195(t10429: f64, t1358: f64, t2299: f64, t488: f64, t2268: f64, t27102: f64, t6316: f64, t10249: f64, t6313: f64, t10151: f64, t10170: f64, t1063: f64, t1349: f64, t1353: f64, t161: f64, t2343: f64, t30113: f64, t30118: f64, t30120: f64, t30123: f64, t31974: f64, t31984: f64, t31988: f64, t31990: f64, t31994: f64, t4324: f64) -> f64 {
    let t31998 = 0.63233348079280332442e-2_f64 * t1358 * t2299 * t10429 * t488;
    let t32001 = 0.14227503317838074799e1_f64 * t2268 * t6316 * t27102;
    let t32003 = 0.91056021234163678716e0_f64 * t6313 * t10249;
    let t32004 = t31974 - 0.1138200265427045984e0_f64 * t1063 * t2343 * t10151 * t4324 + 0.63233348079280332442e-2_f64 * t1349 * t10170 * t161 * t1353 + t31984 + t31988 - t31990 - t31994 - t31998 + t32001 + t32003 - t30113 + t30118 + t30120 + t30123;
    t32004
}
