//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1217/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1217<F: Float>(t10845: F, t10864: F, t1268: F, t14514: F, t14523: F, t21355: F, t21362: F, t21369: F, t21877: F, t2265: F, t2923: F, t43164: F, t4334: F, t4342: F, t4965: F, t4969: F, t4973: F, t5457: F, t5468: F, t82112: F, t88149: F, t88153: F, t88184: F, t91330: F, t992: F) -> F {
    let t91387 = F::new(12.0) * t2265 * t14523 * t21877 - F::new(4.0) / F::new(3.0) * t2265 * t14514 * t91330 - F::new(4.0) / F::new(3.0) * t2265 * t2923 * t82112 * t992 - F::new(2.0) / F::new(3.0) * t2265 * t10845 * t4965 * t5468 - F::new(2.0) * t2265 * t4334 * t88184 - F::new(2.0) * t2265 * t2923 * t4973 * t5468 - F::new(4.0) / F::new(3.0) * t2265 * t2923 * t21369 * t1268 - F::new(4.0) / F::new(3.0) * t2265 * t4342 * t88149 + F::new(2.0) / F::new(9.0) * t2265 * t4334 * t88153 + F::new(4.0) * t2265 * t2923 * t4969 * t5468 + F::new(2.0) * t2265 * t43164 * t4965 * t5457 + F::new(8.0) / F::new(3.0) * t2265 * t10845 * t21355 * t1268 - F::new(8.0) * t2265 * t2923 * t21362 * t1268 - F::new(12.0) * t2265 * t10864 * t4969 * t5457;
    t91387
}
