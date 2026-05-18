//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1019/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1019<F: Float>(t1444: F, t32211: F, t5673: F, t32206: F, t1032: F, t8578: F, t1426: F, t786: F, t545: F, t72: F, t686: F, t7063: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32213 = t5673 * t32211 * t1444;
    let t32214 = t32206 * t32213;
    let t32216 = t8578 * t1032;
    let t32217 = t32216 * t1426;
    let t32218 = t786 * t32217;
    let t32219 = t545 * t72;
    let t32220 = t32219 * t686;
    let t32222 = F::new(0.14456046980341999104e-1) * t32218 * t32220;
    let t32223 = t7063 * t32217;
    (t32213, t32214, t32216, t32217, t32218, t32219, t32220, t32222, t32223)
}
