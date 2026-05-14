//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 587/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk587<F: Float>(t2105: F, t3696: F, t1543: F, t3573: F, t3659: F, t3704: F, t3711: F, t5668: F, t5673: F, t5678: F, t5682: F, t5691: F, t5693: F, t5731: F, t5733: F, t5736: F, t5739: F, t5742: F, t5746: F) -> (F, F, F) {
    let t5770 = t3696 * t2105;
    let t5771 = t5770 * t1543;
    let t5788 = -0.1294625e1 * t5691 + 0.258925e1 * t5693 + t3704 + 0.10064166666666666667e0 * t3573 + 0.10064166666666666667e0 * t5668 - 0.20128333333333333333e0 * t5673 + 0.60385e0 * t5678 - 0.60385e0 * t5682 + 0.82524375e-1 * t5731 + 0.16504875e0 * t5733 + t3711 + 0.5519e-1 * t3659 + 0.5519e-1 * t5736 - 0.27595e-1 * t5739 + 0.16557e0 * t5742 - 0.16557e0 * t5746;
    (t5770, t5771, t5788)
}
