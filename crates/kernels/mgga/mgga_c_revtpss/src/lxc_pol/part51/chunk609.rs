//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 609/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk609<F: Float>(t7143: F, t7150: F, t1976: F, t999: F, t7145: F, t1071: F, t1982: F, t3268: F, t359: F) -> (F, F, F, F, F) {
    let t7151 = t7150 * t7143;
    let t7152 = t1976 * t999;
    let t7153 = t7145 * t7152;
    let t7156 = t1982 * t1071;
    let t7159 = t1982 * t7143;
    let t7160 = t3268 * t359;
    (t7151, t7153, t7156, t7159, t7160)
}
