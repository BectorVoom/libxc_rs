//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1370/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1370(t11986: f64, t1305: f64, t12000: f64, t599: f64, t475: f64, t11981: f64, t1265: f64, t1063: f64, t1064: f64, t11977: f64, t11978: f64, t12018: f64, t1266: f64, t2268: f64, t2343: f64, t30126: f64, t30129: f64, t30132: f64, t30135: f64, t30145: f64, t30148: f64, t32008: f64, t32010: f64, t535: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t38388 = t11986 * t1305;
    let t38392 = t599 * t12000;
    let t38393 = t38392 * t475;
    let t38399 = t11981 * t1265;
    let t38409 = t30126 + t30129 - t30132 + t32008 + t32010 + t30135 + 0.28455006635676149599e-1_f64 * t1063 * t1064 * t38388 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t38393 - 0.1707300398140568976e0_f64 * t6305 * t11978 + 0.56910013271352299198e-1_f64 * t2268 * t2343 * t38399 + 0.56910013271352299198e-1_f64 * t2268 * t535 * t12018 - 0.85365019907028448797e-1_f64 * t2268 * t11977 * t1266 - t30145 + t30148;
    (t38388, t38392, t38393, t38399, t38409)
}
