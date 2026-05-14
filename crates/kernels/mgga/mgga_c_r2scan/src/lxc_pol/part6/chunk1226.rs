//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1226/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1226<F: Float>(t1551: F, t560: F, t6085: F, t6086: F, t20526: F, t6093: F, t20565: F, t2147: F, t2155: F, t6063: F, t1632: F, t359: F) -> (F, F, F, F, F) {
    let t22692 = t1551 * t560;
    let t22694 = t6085 * t6086 * t22692;
    let t22697 = t6093 * t6086 * t20526;
    let t22700 = t2147 * t6086 * t20565;
    let t22703 = t2155 * t6063 * t20526;
    let t22709 = t359 * t1632;
    (t22694, t22697, t22700, t22703, t22709)
}
