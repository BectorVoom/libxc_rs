//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1374/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1374<F: Float>(t11986: F, t1305: F, t12000: F, t599: F, t475: F, t11981: F, t1265: F, t1063: F, t1064: F, t11977: F, t11978: F, t12018: F, t1266: F, t2268: F, t2343: F, t30126: F, t30129: F, t30132: F, t30135: F, t30145: F, t30148: F, t32008: F, t32010: F, t535: F, t6305: F) -> (F, F, F, F, F) {
    let t38388 = t11986 * t1305;
    let t38392 = t599 * t12000;
    let t38393 = t38392 * t475;
    let t38399 = t11981 * t1265;
    let t38409 = t30126 + t30129 - t30132 + t32008 + t32010 + t30135 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t1064 * t38388 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t2343 * t38393 - F::cast_from(0.1707300398140568976e0_f64) * t6305 * t11978 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t38399 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t535 * t12018 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t11977 * t1266 - t30145 + t30148;
    (t38388, t38392, t38393, t38399, t38409)
}
