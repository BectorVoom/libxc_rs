//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1152/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1152<F: Float>(t10781: F, t7535: F, t10757: F, t980: F, t10761: F, t26278: F, t1054: F, t2133: F, t8093: F, t26176: F, t37717: F, t26150: F, t37720: F) -> (F, F, F, F, F, F) {
    let t39814 = t10781 * t7535;
    let t39816 = t980 * t10757;
    let t39818 = t26278 * t10761;
    let t39821 = t2133 * t1054 * t8093;
    let t39823 = t37717 * t26176;
    let t39824 = F::cast_from(0.47609969197673950972e-2_f64) * t39823;
    let t39825 = t37720 * t26150;
    (t39814, t39816, t39818, t39821, t39824, t39825)
}
