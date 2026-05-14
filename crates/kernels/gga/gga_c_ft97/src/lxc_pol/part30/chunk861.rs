//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 861/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk861<F: Float>(t7584: F, t870: F, t10696: F, t7672: F, t7662: F, t848: F, t668: F, t7679: F, t34199: F, t8392: F, t34160: F, t34209: F, t1882: F, t34091: F, t34115: F, t34232: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t143612 = t870 * t7584;
    let t143621 = t10696 * t7672;
    let t143653 = t848 * t7662;
    let t143660 = t7679 * t668;
    let t143673 = t7672 * t668;
    let t143718 = t8392 * t34199;
    let t143720 = t8392 * t34160;
    let t143722 = t8392 * t34209;
    let t143753 = t1882 * t34091;
    let t143789 = t1882 * t34115;
    let t143823 = t1882 * t34232;
    (t143612, t143621, t143653, t143660, t143673, t143718, t143720, t143722, t143753, t143789, t143823)
}
