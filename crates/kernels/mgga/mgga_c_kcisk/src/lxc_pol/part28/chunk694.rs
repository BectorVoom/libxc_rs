//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 694/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk694<F: Float>(t4726: F, t8510: F, t26: F, t1659: F, t8514: F, t8518: F, t4711: F, t4723: F, t6756: F, t6823: F, t8512: F, t8516: F, t8520: F, t8525: F, t8527: F, t8559: F, t8561: F) -> (F, F, F, F, F, F, F) {
    let t8564 = t4726 * t8510;
    let t8565 = t26 * t8564;
    let t8567 = t1659 * t8514;
    let t8568 = t26 * t8567;
    let t8570 = t1659 * t8518;
    let t8571 = t26 * t8570;
    let t8573 = -0.9494625e0 * t8525 + 0.1898925e1 * t8527 + t4711 + 0.19931111111111111111e0 * t6756 - 0.19931111111111111111e0 * t8512 + 0.59793333333333333334e0 * t8516 - 0.29896666666666666667e0 * t8520 + 0.15358125e0 * t8559 + 0.3071625e0 * t8561 + t4723 + 0.10954222222222222222e0 * t6823 - 0.27385555555555555556e-1 * t8565 + 0.16431333333333333333e0 * t8568 - 0.82156666666666666667e-1 * t8571;
    (t8564, t8565, t8567, t8568, t8570, t8571, t8573)
}
