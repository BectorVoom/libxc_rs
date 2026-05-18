//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 605/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk605<F: Float>(t3488: F, t969: F, t825: F, t1445: F, t3447: F, t833: F, t3309: F, t2685: F, t2684: F, t3009: F, t935: F, t2087: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3489 = t969 * t3488;
    let t3490 = t825 * t3489;
    let t3491 = F::new(0.19171462976960374838e0) * t3490;
    let t3492 = t1445 * t3447;
    let t3494 = F::new(0.11502877786176224903e2) * t833 * t3492;
    let t3499 = F::new(0.15976219147466979032e-1) * t3309;
    let t3500 = t2685 * t3488;
    let t3501 = t2684 * t3500;
    let t3502 = F::new(0.19171462976960374838e0) * t3501;
    let t3503 = t3009 * t935;
    let t3504 = t1445 * t3503;
    let t3506 = F::new(0.69017266717057349418e1) * t2087 * t3504;
    (t3489, t3491, t3492, t3494, t3499, t3500, t3502, t3503, t3504, t3506)
}
