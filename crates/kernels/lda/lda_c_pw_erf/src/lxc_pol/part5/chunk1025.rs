//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1025<F: Float>(t3965: F, t6492: F, t6762: F, t6352: F, t6766: F, t21397: F, t348: F, t12439: F, t4488: F, t34: F, t6711: F, t12362: F, t4494: F, t21387: F, t21388: F, t21389: F, t21390: F, t21392: F, t21393: F, t21396: F, t21401: F, t21403: F) -> (F, F, F, F, F, F, F) {
    let t21406 = 16.0 / 5.0 * t3965 * t6762 * t6492;
    let t21409 = 16.0 / 3.0 * t3965 * t6766 * t6352;
    let t21410 = t21397 * t348;
    let t21413 = 8.0 / 3.0 * t4488 * t12439 * t21410;
    let t21414 = t6711 * t34;
    let t21417 = 32.0 / 15.0 * t12362 * t4494 * t21414;
    let t21418 = t21387 + t21388 + t21389 + t21390 - t21392 + t21393 - t21396 + t21401 - t21403 + t21406 - t21409 + t21413 + t21417;
    (t21406, t21409, t21410, t21413, t21414, t21417, t21418)
}
