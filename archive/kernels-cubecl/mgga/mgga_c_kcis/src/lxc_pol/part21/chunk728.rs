//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 728/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk728<F: Float>(t1282: F, t1291: F, t187: F, t2205: F, t3664: F, t3669: F, t437: F, t7739: F, t7741: F, t7742: F, t7745: F, t7767: F, t7807: F, t7809: F, t7812: F, t7823: F) -> F {
    let t7827 = t7739 - t7741 - t7742 + t7745 - t7767 + t187 * (-t1282 * t7823 - t1291 * t7809 - t2205 * t3664 + F::cast_from(2.0_f64) * t3669 * t7812 + t437 * t7807 - t7739 + t7741 + t7742 - t7745 + t7767);
    t7827
}
