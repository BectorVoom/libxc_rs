//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 386/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk386<F: Float>(t1410: F, t252: F, t1312: F, t1317: F, t1324: F, t1331: F, t1338: F, t1343: F, t1385: F, t1388: F, t1396: F, t1399: F, t1407: F, t1409: F) -> (F, F) {
    let t1412 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t252 * t1410;
    let t1413 = -t1312 - t1317 + t1324 + t1331 - t1338 - t1343 - t1385 + t1388 + t1396 + t1399 + t1407 + t1409 - t1412;
    (t1412, t1413)
}
