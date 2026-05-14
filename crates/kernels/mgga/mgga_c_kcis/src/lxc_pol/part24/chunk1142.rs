//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1142/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1142<F: Float>(t14402: F, t93426: F, t95915: F, t1071: F, t1709: F, t4547: F, t95830: F, t100297: F, t100301: F, t100307: F, t100312: F, t100340: F, t100972: F, t93425: F, t93471: F, t95816: F, t95817: F) -> (F, F, F) {
    let t101136 = t93426 * t95915 * t14402;
    let t101141 = t95830 * t1709 * t1071 * t4547;
    let t101146 = 0.51485339506172839507e-4 * t93471 - 0.33163888888888888888e-2 * t100297 - 0.16581944444444444444e-2 * t100301 - 0.55273148148148148147e-3 * t100307 - 0.16581944444444444444e-2 * t100312 - 0.61836467013888888889e-4 * t93425 * t100972 - 0.61836467013888888889e-4 * t93425 * t101136 - 0.12367293402777777778e-3 * t93425 * t101141 + 0.49745833333333333332e-2 * t100340 - t95816 - 0.7369753086419753086e-3 * t95817;
    (t101136, t101141, t101146)
}
