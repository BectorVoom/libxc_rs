//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 934/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk934<F: Float>(t22908: F, t4606: F, t22907: F, t4431: F, t5691: F, t1564: F, t446: F, t22986: F, t4417: F, t7793: F, t25955: F, t920: F, t29569: F, t469: F, t1317: F, t28: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29615 = t22908 * t4606;
    let t29616 = t22907 * t29615;
    let t29621 = t5691 * t4431;
    let t29622 = t1564 * t29621;
    let t29623 = t446 * t29622;
    let t29625 = t22986 * t4417;
    let t29626 = t7793 * t29625;
    let t29627 = t446 * t29626;
    let t29629 = t25955 * t920;
    let t29630 = t1564 * t29629;
    let t29631 = t446 * t29630;
    let t29633 = t469 * t29569;
    let t29635 = t1317 * t28 * t29633;
    (t29615, t29616, t29622, t29623, t29626, t29627, t29630, t29631, t29633, t29635)
}
