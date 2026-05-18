//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1369/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1369<F: Float>(t1063: F, t11983: F, t1306: F, t1624: F, t2268: F, t30091: F, t30094: F, t30096: F, t30098: F, t30103: F, t31900: F, t31906: F, t31909: F, t31912: F, t31915: F, t31919: F, t3691: F, t3701: F, t6305: F) -> F {
    let t38382 = -t31900 - t31906 - t31909 - F::new(0.28455006635676149599e-1) * t1063 * t3701 * t1306 + F::new(0.28455006635676149599e-1) * t2268 * t1624 * t3691 + F::new(0.1138200265427045984e0) * t6305 * t11983 + t31912 + t31915 + t30091 + t30094 + t30096 - t31919 + t30098 + t30103;
    t38382
}
