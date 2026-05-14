//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1438/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1438<F: Float>(t25569: F, t6086: F, t6093: F, t2147: F, t25573: F, t6395: F, t8232: F, t146: F, t2145: F, t2832: F, t2151: F, t6103: F, t980: F, t5100: F, t8071: F, t6407: F) -> (F, F, F, F, F, F, F) {
    let t27058 = t6093 * t6086 * t25569;
    let t27061 = t2147 * t6086 * t25573;
    let t27063 = t6395 * t8232;
    let t27067 = t146 * t2145 * t2832;
    let t27068 = t27067 * t2151;
    let t27074 = t980 * t6103;
    let t27077 = t5100 * t8071;
    let t27078 = 0.4939086887201633699e-1 * t27077;
    let t27079 = t6407 * t8071;
    (t27058, t27061, t27063, t27068, t27074, t27078, t27079)
}
