//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 675/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk675<F: Float>(t1982: F, t8507: F, t1984: F, t359: F, t1981: F, t338: F, t3056: F) -> (F, F, F, F) {
    let t8508 = t1982 * t8507;
    let t8509 = t1984 * t359;
    let t8512 = t1981 * t338;
    let t8513 = t8512 * t3056;
    (t8508, t8509, t8512, t8513)
}
