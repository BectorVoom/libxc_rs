//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1072/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1072<F: Float>(t122003: F, t27186: F, t34049: F, t686: F, t72: F, t32474: F, t32469: F, t119992: F, t120003: F, t120014: F, t120017: F, t121942: F, t121946: F, t121975: F, t1579: F, t27350: F, t31812: F, t32440: F, t8649: F) -> (F,) {
    let t127794 = t122003 * t27186;
    let t127798 = t34049 * t72 * t686;
    let t127799 = t32474 * t127798;
    let t127801 = t32469 * t127798;
    let t127807 = -0.17135921299530705785e1 * t8649 * t31812 * t32440 * t1579 + 0.25702851531048074406e-1 * t127794 - 0.14456046980341999104e-1 * t121942 + t121946 + t119992 + 0.25389723392137995738e-1 * t127799 - 0.14279934416275588154e-1 * t127801 - t120003 + 0.37645955677973955999e-4 * t120014 - 0.66934509195437693771e-4 * t120017 - 0.17347256376410398924e1 * t121975 * t27350;
    (t127807,)
}
