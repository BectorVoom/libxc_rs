//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1119/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1119<F: Float>(t10627: F, t1858: F, t787: F, t107: F, t548: F, t734: F, t2365: F, t24474: F, t7390: F, t32514: F, t7584: F, t7585: F, t1980: F, t8792: F, t10024: F, t10843: F, t2013: F) -> (F, F, F, F, F, F, F) {
    let t32743 = t1858 * t10627;
    let t32744 = t787 * t32743;
    let t32745 = t107 * t548;
    let t32748 = 0.79445533226334281486e-1 * t32744 * t32745 * t734;
    let t32752 = t7390 * t2365 * t24474;
    let t32753 = 0.29792074959875355558e-1 * t32752;
    let t32756 = 0.87421871174939309262e2 * t7584 * t7585 * t32514;
    let t32757 = t1980 * t8792;
    let t32758 = t32757 * t10024;
    let t32759 = 0.89376224879626066674e-1 * t32758;
    let t32760 = t2013 * t10843;
    (t32745, t32748, t32753, t32756, t32757, t32759, t32760)
}
