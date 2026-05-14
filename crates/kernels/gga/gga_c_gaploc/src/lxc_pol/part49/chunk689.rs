//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 689/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk689<F: Float>(t123: F, t3431: F, t883: F, t969: F, t825: F, t2685: F, t2684: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13063 = t3431 * t123;
    let t13064 = t13063 * t883;
    let t13065 = t969 * t13064;
    let t13066 = t825 * t13065;
    let t13069 = t2685 * t13064;
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13074 = 0.89376224879626066675e-1 * t13073;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1 * t13078;
    let t13086 = -3.0 / 256.0 * t12555 - 27.0 / 8192.0 * t12558 + 27.0 / 524288.0 * t12561 - 9.0 / 524288.0 * t12564 + 9.0 / 8192.0 * t12566 + t12569 / 256.0;
    (t13063, t13064, t13065, t13066, t13069, t13070, t13072, t13074, t13077, t13079, t13086)
}
