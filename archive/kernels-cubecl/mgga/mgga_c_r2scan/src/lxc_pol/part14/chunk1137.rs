//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1137/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1137<F: Float>(t10772: F, t3308: F, t7945: F, t10781: F, t7535: F, t10757: F, t980: F, t10761: F, t26278: F, t1054: F, t2133: F, t8093: F) -> (F, F, F, F, F) {
    let t39807 = t10772 * t3308 * t7945;
    let t39814 = t10781 * t7535;
    let t39816 = t980 * t10757;
    let t39818 = t26278 * t10761;
    let t39821 = t2133 * t1054 * t8093;
    (t39807, t39814, t39816, t39818, t39821)
}
