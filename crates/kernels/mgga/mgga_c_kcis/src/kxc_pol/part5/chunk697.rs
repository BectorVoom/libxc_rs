//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 697/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk697<F: Float>(t5498: F, t5499: F, t1319: F, t1897: F, t1317: F, t1958: F, t3820: F, t5481: F, t3795: F, t3833: F, t5469: F, t5472: F, t5475: F, t5479: F, t1410: F, t3821: F, t3824: F, t456: F) -> (F, F, F, F, F, F, F, F) {
    let t5500 = t5498 * t5499;
    let t5503 = t1897 * t1319;
    let t5510 = t1317 * t1958;
    let t5513 = t3820 * t1897;
    let t5514 = t5513 * t1319;
    let t5516 = t1317 * t5481;
    let t5523 = -0.991e-2 * t5514 + 0.1982e-1 * t5516 + t3833 + 0.13758333333333333333e-2 * t3795 + 0.13758333333333333333e-2 * t5469 - 0.27516666666666666667e-2 * t5472 + 0.8255e-2 * t5475 + 0.8255e-2 * t5479;
    let t5526 = -t3821 * t5503 / 8.0 + t3824 * t1897 / 4.0 + t1410 * t5481 / 4.0 + t5510 * t1319 / 4.0 + t456 * t5523 / 2.0;
    (t5500, t5503, t5510, t5513, t5514, t5516, t5523, t5526)
}
