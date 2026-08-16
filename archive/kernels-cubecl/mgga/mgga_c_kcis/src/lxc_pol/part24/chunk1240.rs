//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1240/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1240<F: Float>(t1020: F, t18530: F, t7718: F, t1856: F, t26996: F, t5329: F, t5336: F, t1267: F, t30066: F, t6774: F, t26975: F, t5341: F) -> (F, F, F, F) {
    let t100229 = t1020 * t7718 * t18530;
    let t100235 = t5329 * t26996 * t5336 * t1856;
    let t100244 = t5329 * t30066 * t6774 * t1267;
    let t100257 = t5329 * t26975 * t1856 * t5341;
    (t100229, t100235, t100244, t100257)
}
