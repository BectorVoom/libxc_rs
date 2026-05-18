//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1010/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1010<F: Float>(t1757: F, t535: F, t6446: F, t1835: F, t209: F, t6447: F, t508: F, t6451: F, t6455: F, t1810: F, t10194: F, t31: F, t4: F) -> (F, F, F, F, F) {
    let t22403 = F::new(8.0) * t1757 * t535 * t6446;
    let t22406 = F::new(0.71233333333333333333e-1) * t209 * t1835 * t6447;
    let t22410 = F::new(0.36845452142031360636e2) * t209 * t508 * t6451 * t6455;
    let t22411 = t1810 * t1810;
    let t22417 = F::new(0.11483710345679012345e-1) * t4 * t10194 * t31;
    (t22403, t22406, t22410, t22411, t22417)
}
