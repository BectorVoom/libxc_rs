//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1070/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1070<F: Float>(t468: F, t4715: F, t735: F, t378: F, t5002: F, t1398: F, t1524: F, t410: F, t5031: F, t1376: F, t1509: F, t41: F, t1751: F, t4965: F, t5006: F, t1527: F, t5234: F) -> (F, F, F, F, F, F, F, F) {
    let t18872 = 0.67471172535210825684e-1 * t735 * t4715 * t468;
    let t18875 = 0.21687162600603479684e-1 * t735 * t378 * t5002;
    let t18878 = 0.86748650402413918736e-1 * t735 * t1398 * t1524;
    let t18879 = t410 * t5031;
    let t18882 = t41 * t1376 * t1509;
    let t18884 = t1751 * t4965;
    let t18888 = 0.38025319932552508021e2 * t735 * t378 * t5006;
    let t18889 = t5234 * t1527;
    (t18872, t18875, t18878, t18879, t18882, t18884, t18888, t18889)
}
