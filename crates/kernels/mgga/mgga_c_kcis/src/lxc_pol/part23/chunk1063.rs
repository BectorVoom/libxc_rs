//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1063/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1063<F: Float>(t11418: F, t1386: F, t1466: F, t491: F, t1494: F, t1598: F, t37622: F, t1014: F, t27391: F, t27345: F, t7895: F, t27348: F, t18210: F, t27341: F, t2237: F, t11881: F, t7925: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94408 = t1386 * t11418;
    let t94424 = t1466 * t491;
    let t94425 = t94424 * t1494;
    let t94440 = t37622 * t1598;
    let t94451 = t1014 * t27391;
    let t94465 = t7895 * t27345;
    let t94467 = t7895 * t27348;
    let t94469 = t18210 * t27341;
    let t94470 = t2237 * t94469;
    let t94472 = t11881 * t7925;
    (t94408, t94424, t94425, t94440, t94451, t94465, t94467, t94469, t94470, t94472)
}
