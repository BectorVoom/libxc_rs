//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 898/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk898<F: Float>(t16795: F, t4162: F, t5661: F, t3723: F, t5752: F, t1464: F, t12251: F, t1364: F, t1387: F, t16682: F, t16739: F, t16744: F, t16754: F, t16756: F, t16759: F, t16763: F, t16766: F, t16769: F, t16775: F, t16780: F, t16785: F, t16791: F, t16793: F, t3718: F, t3964: F, t5742: F, t5886: F) -> (F, F, F) {
    let t16796 = t4162 * t16795;
    let t16797 = t5661 * t16796;
    let t16799 = t5752 * t3723;
    let t16800 = t1464 * t16799;
    let t16802 = -F::new(0.3684876543209876543e-3) * t16739 + F::new(0.66725e-1) * t5742 * t3718 + F::new(0.14739506172839506172e-2) * t12251 - F::new(0.13345e0) * t16744 * t1387 + F::new(0.13345e0) * t3964 * t5886 + F::new(0.13345e0) * t1364 * t16682 - F::new(0.49745833333333333332e-2) * t16754 - F::new(0.3684876543209876543e-3) * t16756 - F::new(0.88437037037037037034e-2) * t16759 - F::new(0.44218518518518518517e-2) * t16763 + F::new(0.16581944444444444444e-2) * t16766 + t16769 + F::new(0.66327777777777777776e-2) * t16775 - F::new(0.44218518518518518517e-2) * t16780 + F::new(0.13265555555555555555e-1) * t16785 - F::new(0.11054629629629629629e-1) * t16791 - F::new(0.22109259259259259258e-2) * t16793 - F::new(0.27636574074074074073e-2) * t16797 + F::new(0.16581944444444444444e-2) * t16800;
    (t16797, t16800, t16802)
}
