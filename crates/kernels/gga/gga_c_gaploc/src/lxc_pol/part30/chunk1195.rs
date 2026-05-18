//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1195/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1195<F: Float>(t10429: F, t1358: F, t2299: F, t488: F, t2268: F, t27102: F, t6316: F, t10249: F, t6313: F, t10151: F, t10170: F, t1063: F, t1349: F, t1353: F, t161: F, t2343: F, t30113: F, t30118: F, t30120: F, t30123: F, t31974: F, t31984: F, t31988: F, t31990: F, t31994: F, t4324: F) -> F {
    let t31998 = F::new(0.63233348079280332442e-2) * t1358 * t2299 * t10429 * t488;
    let t32001 = F::new(0.14227503317838074799e1) * t2268 * t6316 * t27102;
    let t32003 = F::new(0.91056021234163678716e0) * t6313 * t10249;
    let t32004 = t31974 - F::new(0.1138200265427045984e0) * t1063 * t2343 * t10151 * t4324 + F::new(0.63233348079280332442e-2) * t1349 * t10170 * t161 * t1353 + t31984 + t31988 - t31990 - t31994 - t31998 + t32001 + t32003 - t30113 + t30118 + t30120 + t30123;
    t32004
}
