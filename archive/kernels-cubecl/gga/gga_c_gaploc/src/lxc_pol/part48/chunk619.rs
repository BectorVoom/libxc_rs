//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 619/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk619<F: Float>(t11172: F, t447: F, t2343: F, t550: F, t4261: F, t158: F, t3516: F, t123: F, t488: F, t3529: F, t10186: F, t1063: F, t11169: F, t1358: F, t3519: F, t3537: F, t3542: F, t3546: F, t380: F, t419: F) -> (F, F, F, F, F, F, F) {
    let t11173 = t11172 * t447;
    let t11174 = t2343 * t11173;
    let t11177 = t550 * t11172;
    let t11178 = t4261 * t11177;
    let t11181 = t158 * t3516;
    let t11182 = t11181 * t123;
    let t11183 = t11182 * t488;
    let t11186 = t158 * t3529;
    let t11187 = t11186 * t123;
    let t11188 = t11187 * t488;
    let t11202 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t11169 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t11174 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t11178 - F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t11183 - F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t11188 + F::cast_from(0.94850022118920498664e-2_f64) * t10186 - F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3546 + F::cast_from(0.7588001769513639893e-1_f64) * t380 * t3542 - F::cast_from(0.1138200265427045984e0_f64) * t380 * t3537 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3519 - F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3546;
    (t11173, t11177, t11181, t11182, t11186, t11187, t11202)
}
