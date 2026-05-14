//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 630/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk630<F: Float>(t12658: F, t3005: F, t3295: F, t9800: F, t11053: F, t9805: F, t1029: F, t9796: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F, t9824: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13050 = 0.11502877786176224903e1 * t12658;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13054 = 0.19171462976960374838e1 * t13053;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13057 = 0.11502877786176224903e1 * t13056;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1 * t13078;
    let t13086 = -3.0 / 256.0 * t12555 - 27.0 / 8192.0 * t12558 + 27.0 / 524288.0 * t12561 - 9.0 / 524288.0 * t12564 + 9.0 / 8192.0 * t12566 + t12569 / 256.0;
    (t13050, t13052, t13054, t13055, t13057, t13058, t13059, t13072, t13073, t13077, t13079, t13086)
}
