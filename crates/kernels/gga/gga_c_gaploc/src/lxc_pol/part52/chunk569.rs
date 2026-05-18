//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 569/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk569<F: Float>(t10898: F, t8516: F, t959: F, t10847: F, t7573: F, t7572: F, t10820: F, t326: F, t825: F, t7585: F, t2684: F, t1: F, t2084: F) -> (F, F, F, F, F, F, F) {
    let t10899 = F::new(0.42603251059911944084e-1) * t10898;
    let t10900 = t8516 * t959;
    let t10901 = F::new(0.14896037479937677779e-1) * t10900;
    let t10903 = t7573 * t10847;
    let t10905 = F::new(0.69017266717057349418e1) * t7572 * t10903;
    let t10906 = t326 * t10820;
    let t10908 = F::new(0.92023022289409799224e1) * t825 * t10906;
    let t10909 = t7585 * t10820;
    let t10911 = F::new(0.43710935587469654631e2) * t2684 * t10909;
    let t10912 = t2084 * t1;
    (t10899, t10900, t10901, t10905, t10908, t10911, t10912)
}
