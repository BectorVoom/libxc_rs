//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1137/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1137(t10772: f64, t3308: f64, t7945: f64, t10781: f64, t7535: f64, t10757: f64, t980: f64, t10761: f64, t26278: f64, t1054: f64, t2133: f64, t8093: f64) -> (f64, f64, f64, f64, f64) {
    let t39807 = t10772 * t3308 * t7945;
    let t39814 = t10781 * t7535;
    let t39816 = t980 * t10757;
    let t39818 = t26278 * t10761;
    let t39821 = t2133 * t1054 * t8093;
    (t39807, t39814, t39816, t39818, t39821)
}
