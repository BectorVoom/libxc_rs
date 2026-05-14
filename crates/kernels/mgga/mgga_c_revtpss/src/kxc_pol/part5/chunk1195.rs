//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1195/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1195<F: Float>(t20795: F, t5352: F, t3720: F, t3153: F, t6622: F, t5341: F, t5333: F, t1263: F, t6587: F, t1122: F, t1042: F, t3172: F, t6624: F, t1247: F, t1032: F, t6564: F) -> (F, F, F, F, F, F, F) {
    let t20796 = t20795 * t5352;
    let t20797 = t3720 * t20796;
    let t20800 = t6622 * t3153;
    let t20801 = t20800 * t5341;
    let t20802 = t3720 * t20801;
    let t20805 = t20800 * t5333;
    let t20806 = t3720 * t20805;
    let t20809 = t1263 * t6587;
    let t20810 = t20809 * t1122;
    let t20811 = t1042 * t20810;
    let t20816 = t3172 * t6624;
    let t20817 = t1247 * t20816;
    let t20819 = t6564 * t1032;
    (t20797, t20800, t20802, t20806, t20811, t20817, t20819)
}
