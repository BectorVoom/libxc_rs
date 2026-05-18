//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1108/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1108<F: Float>(t21499: F, t1066: F, t154: F, t18060: F, t276: F, t735: F, t7620: F, t17867: F, t2104: F, t2911: F, t2064: F, t2922: F, t2924: F) -> (F, F, F, F, F) {
    let t21500 = F::new(0.28582678745379824648e-3) * t21499;
    let t21538 = t276 * t154 * t18060 * t1066;
    let t21542 = t735 * t7620;
    let t21543 = t21542 / F::new(54.0);
    let t21623 = t2104 * t17867 * t2911;
    let t21624 = F::new(0.28582678745379824648e-3) * t21623;
    let t21626 = t2922 * t2064 * t2924;
    (t21500, t21538, t21543, t21624, t21626)
}
