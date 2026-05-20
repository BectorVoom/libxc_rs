//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1023/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1023<F: Float>(t2322: F, t8461: F, t4254: F, t1310: F, t8460: F, t651: F, t4147: F, t7311: F, t6972: F, t8441: F, t8621: F, t5523: F) -> (F, F, F, F, F, F, F, F) {
    let t32106 = t2322 * t8461;
    let t32107 = F::new(2.0) * t32106;
    let t32108 = t4254 * t8461;
    let t32109 = F::new(2.0) * t32108;
    let t32110 = t1310 * t8460;
    let t32111 = t651 * t32110;
    let t32112 = F::new(2.0) * t32111;
    let t32113 = t4147 * t7311;
    let t32151 = t8621 * t8441 * t6972;
    let t32175 = t2322 * t8460;
    let t32176 = F::new(2.0) * t32175;
    let t32177 = t5523 * t8460;
    (t32107, t32109, t32110, t32112, t32113, t32151, t32176, t32177)
}
