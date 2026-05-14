//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1170/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1170<F: Float>(t33627: F, t6066: F, t7630: F, t2033: F, t2365: F, t2610: F, t8720: F, t15349: F, t3474: F, t10857: F, t5676: F, t10820: F, t15362: F, t1029: F, t7344: F, t7803: F) -> (F, F, F, F, F, F) {
    let t33645 = 0.14300195980740170668e1 * t7630 * t6066 * t33627;
    let t33648 = t2033 * t2365 * t2610 * t8720;
    let t33649 = 0.14896037479937677779e-1 * t33648;
    let t33650 = t15349 * t3474;
    let t33651 = 0.14896037479937677779e-1 * t33650;
    let t33652 = t5676 * t10857;
    let t33653 = 0.29792074959875355558e-1 * t33652;
    let t33656 = 0.85801175884441024006e1 * t15362 * t6066 * t10820;
    let t33658 = t7803 * t1029 * t7344;
    (t33645, t33649, t33651, t33653, t33656, t33658)
}
