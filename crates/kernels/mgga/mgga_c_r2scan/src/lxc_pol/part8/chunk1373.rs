//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1373/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1373<F: Float>(t21649: F, t21659: F, t22114: F, t22116: F, t22125: F, t22130: F, t22132: F, t26520: F, t26522: F, t26525: F, t26528: F, t26532: F, t26535: F, t10241: F, t406: F, t410: F) -> (F, F, F) {
    let t33534 = -0.508088392e-2 * t26520 + 0.79035972088888888887e-2 * t26522 + 0.92286169723947659919e4 * t26525 - t21649 + t22114 - t22116 + 0.2401225740592e-1 * t26528 + t26532 + t26535 - t21659 + t22125 - 0.35089341735807877242e1 * t22130 - 0.31168546390226634765e3 * t22132;
    let t33540 = t406 * t10241;
    let t33542 = t410 * t10241;
    (t33534, t33540, t33542)
}
