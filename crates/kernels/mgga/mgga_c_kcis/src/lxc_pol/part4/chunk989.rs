//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 989/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk989<F: Float>(t3177: F, t3436: F, t1194: F, t381: F, t1095: F, t1169: F, t983: F, t9538: F, t3621: F, t426: F, t187: F, t2997: F) -> (F, F, F, F, F, F, F) {
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    let t10753 = t1095 * t10752;
    let t10787 = t1169 * t983;
    let t10799 = t9538 * t381;
    let t10819 = F::new(1.0) / t3621 / t426;
    let t10845 = t187 * t2997;
    (t10745, t10752, t10753, t10787, t10799, t10819, t10845)
}
