//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 588/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk588<F: Float>(t3025: F, t3470: F, t1022: F, t2610: F, t2365: F, t2033: F, t1457: F, t3447: F, t2103: F, t3451: F, t531: F, t2949: F, t935: F, t1445: F, t813: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3472 = 0.10725146985555128001e1 * t3025 * t3470;
    let t3473 = t2610 * t1022;
    let t3474 = t2365 * t3473;
    let t3475 = t2033 * t3474;
    let t3476 = 0.14896037479937677779e-1 * t3475;
    let t3477 = t1457 * t3447;
    let t3479 = 0.71500979903700853338e0 * t2103 * t3477;
    let t3480 = t531 * t3451;
    let t3483 = t2949 * t935;
    let t3484 = t1445 * t3483;
    let t3486 = 0.46011511144704899612e1 * t813 * t3484;
    let t3487 = t1022 * t123;
    (t3472, t3473, t3474, t3476, t3477, t3479, t3480, t3483, t3484, t3486, t3487)
}
