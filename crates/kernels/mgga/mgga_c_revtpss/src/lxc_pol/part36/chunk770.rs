//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 770/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk770<F: Float>(t532: F, t7933: F, t1450: F, t2014: F, t2034: F, t5542: F, t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7934 = t532 * t7933;
    let t7935 = t7934 * t1450;
    let t7936 = t2014 * t7935;
    let t7937 = t2034 * t5542;
    let t7938 = t2014 * t7937;
    let t7949 = F::new(3.0) * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = F::new(6.0) * t572 * t7950;
    let t7953 = t117 * t7741;
    (t7934, t7935, t7936, t7937, t7938, t7949, t7950, t7952, t7953)
}
