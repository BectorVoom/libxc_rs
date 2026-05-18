//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 838/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk838<F: Float>(t11735: F, t286: F, t686: F, t690: F, t218: F, t2692: F, t777: F, t779: F, t224: F, t2643: F, t709: F, t902: F) -> (F, F, F, F) {
    let t11743 = F::new(0.51947577317044391277e2) * t286 * t686 * t11735 * t690;
    let t11747 = F::new(0.64327917994770140268e2) * t777 * t2692 * t779 * t218;
    let t11748 = t224 * t2643;
    let t11750 = t709 * t902;
    (t11743, t11747, t11748, t11750)
}
