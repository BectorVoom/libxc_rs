//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 770/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk770<F: Float>(t13064: F, t969: F, t825: F, t2685: F, t2684: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13065 = t969 * t13064;
    let t13066 = t825 * t13065;
    let t13069 = t2685 * t13064;
    let t13070 = t2684 * t13069;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13074 = F::cast_from(0.89376224879626066675e-1_f64) * t13073;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = F::cast_from(0.29792074959875355558e-1_f64) * t13078;
    let t13086 = -F::new(3.0) / F::new(256.0) * t12555 - F::new(27.0) / F::new(8192.0) * t12558 + F::new(27.0) / F::new(524288.0) * t12561 - F::new(9.0) / F::new(524288.0) * t12564 + F::new(9.0) / F::new(8192.0) * t12566 + t12569 / F::new(256.0);
    (t13065, t13066, t13069, t13070, t13072, t13074, t13077, t13079, t13086)
}
