//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 969/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk969<F: Float>(t326: F, t43494: F, t825: F, t2684: F, t7585: F, t2365: F, t32261: F, t7390: F, t1: F, t42925: F, t787: F, t43107: F, t723: F) -> (F, F, F, F, F) {
    let t43497 = F::new(0.18404604457881959845e2) * t825 * t326 * t43494;
    let t43500 = F::new(0.14953741122029092374e3) * t2684 * t7585 * t43494;
    let t43502 = t7390 * t2365 * t32261;
    let t43505 = t787 * t42925 * t1;
    let t43508 = t43107 * t723;
    (t43497, t43500, t43502, t43505, t43508)
}
