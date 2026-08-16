//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 898/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk898(t16795: f64, t4162: f64, t5661: f64, t3723: f64, t5752: f64, t1464: f64, t12251: f64, t1364: f64, t1387: f64, t16682: f64, t16739: f64, t16744: f64, t16754: f64, t16756: f64, t16759: f64, t16763: f64, t16766: f64, t16769: f64, t16775: f64, t16780: f64, t16785: f64, t16791: f64, t16793: f64, t3718: f64, t3964: f64, t5742: f64, t5886: f64) -> (f64, f64, f64) {
    let t16796 = t4162 * t16795;
    let t16797 = t5661 * t16796;
    let t16799 = t5752 * t3723;
    let t16800 = t1464 * t16799;
    let t16802 = -0.3684876543209876543e-3_f64 * t16739 + 0.66725e-1_f64 * t5742 * t3718 + 0.14739506172839506172e-2_f64 * t12251 - 0.13345e0_f64 * t16744 * t1387 + 0.13345e0_f64 * t3964 * t5886 + 0.13345e0_f64 * t1364 * t16682 - 0.49745833333333333332e-2_f64 * t16754 - 0.3684876543209876543e-3_f64 * t16756 - 0.88437037037037037034e-2_f64 * t16759 - 0.44218518518518518517e-2_f64 * t16763 + 0.16581944444444444444e-2_f64 * t16766 + t16769 + 0.66327777777777777776e-2_f64 * t16775 - 0.44218518518518518517e-2_f64 * t16780 + 0.13265555555555555555e-1_f64 * t16785 - 0.11054629629629629629e-1_f64 * t16791 - 0.22109259259259259258e-2_f64 * t16793 - 0.27636574074074074073e-2_f64 * t16797 + 0.16581944444444444444e-2_f64 * t16800;
    (t16797, t16800, t16802)
}
