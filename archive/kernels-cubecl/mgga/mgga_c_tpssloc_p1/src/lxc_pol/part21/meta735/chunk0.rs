//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2594/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2594<F: Float>(t15814: F, t225: F, t3030: F, t4940: F, t3623: F, t1009: F, t15425: F, t1243: F, t11712: F, t11880: F, t491: F, t1734: F, t6739: F) -> (F, F, F, F, F, F, F) {
    let t52386 = t15814 * t225;
    let t52434 = t4940 * t3030;
    let t52435 = t52434 * t3623;
    let t52446 = t15425 * t1009;
    let t52447 = t52446 * t1243;
    let t52479 = t11712 * t11880 * t491;
    let t52480 = t1734 * t6739;
    (t52386, t52434, t52435, t52446, t52447, t52479, t52480)
}
