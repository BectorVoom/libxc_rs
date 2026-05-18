//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 922/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk922<F: Float>(t43598: F, t7584: F, t7585: F, t43486: F, t7427: F, t7573: F, t33294: F, t9839: F, t10062: F, t3040: F, t3295: F, t8802: F, t9800: F) -> (F, F, F, F, F) {
    let t43746 = F::new(0.43710935587469654631e2) * t7584 * t7585 * t43598;
    let t43750 = F::new(0.12423108009070322895e3) * t7427 * t7573 * t43486;
    let t43752 = F::new(0.47667319935800568892e0) * t33294 * t9839;
    let t43754 = F::new(0.35750489951850426669e0) * t10062 * t3040;
    let t43756 = t9800 * t8802 * t3295;
    (t43746, t43750, t43752, t43754, t43756)
}
