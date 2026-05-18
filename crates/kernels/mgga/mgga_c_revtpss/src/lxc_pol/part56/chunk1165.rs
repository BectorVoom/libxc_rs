//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1165/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1165<F: Float>(t104115: F, t1936: F, t111734: F, t29427: F, t7002: F, t7334: F, t8245: F, t7331: F, t7696: F, t7953: F, t7950: F, t2170: F, t28271: F) -> (F, F, F, F, F, F, F, F) {
    let t129488 = t104115 * t1936;
    let t129489 = t111734 * t1936;
    let t129490 = t29427 * t7002;
    let t129541 = t8245 * t7334;
    let t129543 = t8245 * t7331;
    let t129555 = t7696 * t7953;
    let t129559 = t7696 * t7950;
    let t129562 = t2170 * t28271;
    (t129488, t129489, t129490, t129541, t129543, t129555, t129559, t129562)
}
