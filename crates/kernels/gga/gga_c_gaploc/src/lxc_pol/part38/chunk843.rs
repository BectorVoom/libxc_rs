//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 843/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk843<F: Float>(t44492: F, t1063: F, t13299: F, t13316: F, t13323: F, t1339: F, t13445: F, t2268: F, t2765: F, t31747: F, t35220: F, t419: F, t44457: F, t44469: F, t44473: F, t44477: F, t44479: F, t44483: F, t44485: F, t44487: F, t44489: F, t44490: F, t44491: F, t448: F, t6313: F, t7937: F) -> F {
    let t44493 = F::new(0.15808337019820083111e-2) * t44492;
    let t44501 = -t44457 - F::new(0.28455006635676149599e-1) * t1063 * t13445 * t448 + F::new(0.7588001769513639893e-1) * t6313 * t13316 - F::new(0.28455006635676149599e-1) * t419 * t13323 + F::new(0.28455006635676149599e-1) * t419 * t13299 - t44469 + t44473 - t44477 + t44479 + t44483 + t44485 + t44487 - t44489 + t44490 + t44491 - t44493 - F::new(0.39837009289946609438e0) * t2268 * t2765 * t35220 + F::new(0.68292015925622759038e0) * t2268 * t7937 * t1339 * t31747;
    t44501
}
