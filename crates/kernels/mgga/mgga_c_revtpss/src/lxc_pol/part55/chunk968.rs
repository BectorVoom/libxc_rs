//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 968/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk968<F: Float>(t33722: F, t8486: F, t30: F, t7782: F, t33: F, t1883: F, t32195: F, t5673: F, t32194: F, t1868: F, t3936: F, t32206: F, t1903: F, t32211: F, t1892: F, t8477: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33723 = t8486 * t33722;
    let t33740 = t30 * t7782;
    let t33888 = t33 * t7782;
    let t33922 = t5673 * t32195 * t1883;
    let t33923 = t32194 * t33922;
    let t33926 = t3936 * t32195 * t1868;
    let t33927 = t32206 * t33926;
    let t33930 = t5673 * t32211 * t1903;
    let t33931 = t32206 * t33930;
    let t33943 = t8477 * t1892;
    (t33723, t33740, t33888, t33922, t33923, t33926, t33927, t33930, t33931, t33943)
}
