//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 396/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk396<F: Float>(t1458: F, t22: F, t1245: F, t197: F, t940: F, t519: F, t1416: F, t1420: F, t1424: F, t1429: F, t1435: F, t1436: F, t1439: F, t1445: F, t1448: F, t1452: F, t1456: F, t256: F) -> (F, F, F, F, F, F) {
    let t1459 = t22 * t1458;
    let t1460 = t197 * t1245;
    let t1461 = t1460 * t940;
    let t1462 = t1459 * t1461;
    let t1464 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t519 * t1462;
    let t1465 = t1416 * t256 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1420 + F::cast_from(0.12155555555555556_f64) * t1424 + t1429 + t1435 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1436 + t1439 - t1445 + t1448 + t1452 + t1456 + t1464;
    (t1459, t1460, t1461, t1462, t1464, t1465)
}
