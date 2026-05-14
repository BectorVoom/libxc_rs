//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1218/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1218<F: Float>(t29393: F, t7904: F, t102357: F, t103303: F, t103394: F, t28369: F, t28388: F, t28551: F, t7908: F, t94472: F, t94489: F, t94492: F, t98489: F, t98491: F, t98519: F, t28426: F, t28544: F) -> (F, F) {
    let t103467 = t29393 * t7904;
    let t103475 = 0.27802083333333333334e-2 * t7908 * t103394 + 0.27802083333333333334e-2 * t7908 * t103303 + 0.37134344353515625001e-4 * t28388 * t103303 - 0.16581944444444444444e-2 * t102357 - 0.23168402777777777778e-3 * t103467 + 0.46336805555555555556e-3 * t28369 * t28551 + t98489 - 0.20594135802469135803e-3 * t98491 - 0.36848765432098765431e-3 * t94472 - 0.15445601851851851852e-3 * t94489 - 0.15445601851851851852e-3 * t94492 - t98519;
    let t103483 = t28544 * t28426;
    (t103475, t103483)
}
