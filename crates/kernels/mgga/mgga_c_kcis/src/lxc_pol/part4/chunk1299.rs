//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1299/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1299<F: Float>(t1396: F, t16665: F, t4123: F, t1464: F, t11914: F, t1364: F, t15978: F, t15987: F, t15989: F, t16612: F, t16615: F, t16620: F, t16625: F, t16628: F, t16629: F, t16632: F, t16636: F, t16640: F, t16644: F, t16651: F, t16656: F, t16661: F, t16663: F) -> (F, F) {
    let t16666 = t1396 * t16665;
    let t16667 = t4123 * t16666;
    let t16668 = t1464 * t16667;
    let t16670 = -t15987 - t15989 - F::new(0.24872916666666666666e-2) * t16612 - F::new(0.55273148148148148147e-3) * t16615 + F::new(0.14739506172839506172e-2) * t16620 + F::new(0.49745833333333333332e-2) * t16625 + t16628 - F::new(0.5895802469135802469e-2) * t16629 - t16632 - F::new(0.73697530864197530861e-3) * t16636 - F::new(0.22109259259259259258e-2) * t16640 - F::new(0.22109259259259259258e-2) * t16644 - F::new(0.22109259259259259258e-2) * t11914 + F::new(0.66725e-1) * t1364 * t15978 + F::new(0.88437037037037037034e-2) * t16651 - F::new(0.16581944444444444444e-2) * t16656 - F::new(0.55273148148148148147e-3) * t16661 - F::new(0.73697530864197530861e-3) * t16663 + F::new(0.99491666666666666664e-2) * t16668;
    (t16668, t16670)
}
