//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1046/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1046<F: Float>(t1984: F, t25586: F, t359: F, t3057: F, t7143: F, t7145: F, t7146: F, t999: F, t1096: F, t7152: F, t7160: F, t1035: F, t8515: F, t1983: F, t1043: F, t7161: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25588 = t1984 * t359 * t25586;
    let t25591 = t3057 * t7143;
    let t25593 = t7145 * t7146 * t999;
    let t25596 = t7152 * t1096;
    let t25597 = t7160 * t25596;
    let t25601 = t7160 * t7146 * t1096;
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25606 = t7161 * t1043;
    (t25588, t25591, t25593, t25596, t25597, t25601, t25604, t25605, t25606)
}
