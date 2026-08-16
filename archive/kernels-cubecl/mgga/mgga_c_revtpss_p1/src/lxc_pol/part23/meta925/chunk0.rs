//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2996/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2996<F: Float>(t1469: F, t1668: F, t66066: F, t19634: F, t78900: F, t11774: F, t53391: F, t6267: F, t23598: F, t999: F, t19380: F, t4866: F, t6258: F) -> (F, F, F, F, F, F) {
    let t79463 = t1469 * t1668 * t66066;
    let t79467 = t78900 * t19634;
    let t79474 = t11774 * t53391 * t6267;
    let t79480 = t23598 * t999;
    let t79500 = t19380 * t1668;
    let t79505 = t6258 * t4866;
    (t79463, t79467, t79474, t79480, t79500, t79505)
}
