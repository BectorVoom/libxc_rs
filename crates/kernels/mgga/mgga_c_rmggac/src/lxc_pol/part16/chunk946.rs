//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 946/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk946<F: Float>(t35256: F, t43096: F, t45736: F, t45738: F, t45742: F, t45744: F, t45746: F, t45750: F, t45752: F, t45754: F, t45757: F, t45759: F, t45763: F, t45767: F, t45775: F, t45777: F, t45779: F) -> (F,) {
    let t48626 = 0.5107751987195740728e-4 * t45736 + 0.2553875993597870364e-4 * t45738 - 0.1440846329149835838e-2 * t35256 + 0.2553875993597870364e-4 * t45742 - 0.5107751987195740728e-4 * t45744 - 0.5107751987195740728e-4 * t45746 + 0.47885174879960069325e-4 * t45750 + 0.212822999466489197e-4 * t45752 - 0.638468998399467591e-4 * t45754 - 0.5107751987195740728e-4 * t45757 - 0.1702583995731913576e-4 * t45759 + 0.1702583995731913576e-4 * t45763 - 0.1064114997332445985e-4 * t45767 + t43096 - 0.54549323308490683461e-1 * t45775 + 0.5107751987195740728e-4 * t45777 + 0.5107751987195740728e-4 * t45779;
    (t48626,)
}
