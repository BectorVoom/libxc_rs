//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 877/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk877<F: Float>(t1835: F, t28381: F, t1842: F, t1856: F, t1659: F, t28373: F, t28389: F, t2063: F, t7718: F) -> (F, F, F, F, F, F, F) {
    let t28645 = t1835 * t28381;
    let t28648 = t1842 * t28381;
    let t28651 = t1856 * t28381;
    let t28654 = t1659 * t28373;
    let t28657 = t1835 * t28373;
    let t28660 = t1842 * t28389;
    let t28663 = t2063 * t7718;
    (t28645, t28648, t28651, t28654, t28657, t28660, t28663)
}
