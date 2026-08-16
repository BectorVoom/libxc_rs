//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1297/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1297(t15437: f64, t24728: f64, t24732: f64, t24658: f64, t27683: f64, t23508: f64, t8026: f64, t7325: f64, t27628: f64, t7324: f64, t15730: f64, t7339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95270 = t15437 * t24728;
    let t95273 = t15437 * t24732;
    let t95295 = t24658 * t27683;
    let t95326 = t8026 * t23508;
    let t95327 = t95326 * t7325;
    let t95332 = t7324 * t27628;
    let t95335 = t7339 * t15730;
    (t95270, t95273, t95295, t95327, t95332, t95335)
}
