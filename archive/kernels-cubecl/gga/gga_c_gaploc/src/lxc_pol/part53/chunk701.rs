//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 701/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk701<F: Float>(t13056: F, t1029: F, t3295: F, t9796: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F) -> (F, F, F, F, F, F, F, F) {
    let t13057 = F::cast_from(0.11502877786176224903e1_f64) * t13056;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = F::cast_from(0.29792074959875355558e-1_f64) * t13078;
    let t13086 = -F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t12555 - F::cast_from(27.0_f64) / F::cast_from(8192.0_f64) * t12558 + F::cast_from(27.0_f64) / F::cast_from(524288.0_f64) * t12561 - F::cast_from(9.0_f64) / F::cast_from(524288.0_f64) * t12564 + F::cast_from(9.0_f64) / F::cast_from(8192.0_f64) * t12566 + t12569 / F::cast_from(256.0_f64);
    (t13057, t13058, t13059, t13072, t13073, t13077, t13079, t13086)
}
