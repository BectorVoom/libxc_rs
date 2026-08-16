//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3077/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3077<F: Float>(t11409: F, t1621: F, t2968: F, t300: F, t3012: F, t11507: F, t15494: F, t11223: F, t379: F, t4930: F, t989: F, t11199: F, t1646: F) -> (F, F, F, F, F, F, F, F) {
    let t52837 = t11409 * t1621;
    let t52840 = t2968 * t1621;
    let t52877 = t300 * t3012;
    let t52894 = t300 * t11507;
    let t52921 = t300 * t15494;
    let t52927 = t11223 * t379;
    let t52994 = t989 * t4930;
    let t53014 = t1646 * t11199;
    (t52837, t52840, t52877, t52894, t52921, t52927, t52994, t53014)
}
