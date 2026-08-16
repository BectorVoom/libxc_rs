//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2023/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2023<F: Float>(t91303: F, t91305: F, t91310: F, t91312: F, t91327: F, t91344: F, t80867: F, t80870: F, t80872: F, t91317: F, t91319: F, t91321: F, t91323: F, t91330: F, t91333: F, t91336: F, t91340: F, t91346: F) -> F {
    let t93720 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91303;
    let t93721 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t91305;
    let t93722 = F::cast_from(0.13457585364713463618e-3_f64) * t91310;
    let t93723 = F::cast_from(0.10541775202358879834e-2_f64) * t91312;
    let t93731 = F::cast_from(0.80745512188280781706e-3_f64) * t91327;
    let t93736 = F::cast_from(0.56521858531796547194e-2_f64) * t91344;
    let t93738 = -t93720 + t93721 + t93722 - t93723 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t80867 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t80870 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t80872 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t91317 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t91319 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t91321 + F::cast_from(0.20186378047070195426e-3_f64) * t91323 + t93731 + F::cast_from(0.33913115119077928316e-1_f64) * t91330 + F::cast_from(0.16956557559538964158e-1_f64) * t91333 - F::cast_from(0.40372756094140390853e-3_f64) * t91336 + F::cast_from(0.24223653656484234512e-2_f64) * t91340 - t93736 + F::cast_from(0.33643963411783659044e-4_f64) * t91346;
    t93738
}
