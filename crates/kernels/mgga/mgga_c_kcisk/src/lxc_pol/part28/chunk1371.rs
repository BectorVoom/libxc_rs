//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1371/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1371<F: Float>(t739: F, t8831: F, t1799: F, t9680: F, t117090: F, t1869: F, t1894: F, t9089: F, t112518: F, t112523: F, t116790: F, t121284: F, t121586: F, t121589: F, t121592: F, t121594: F, t121597: F, t121600: F, t121606: F, t9652: F, t9672: F, t9922: F) -> (F, F, F) {
    let t121609 = t739 * t8831;
    let t121611 = t1799 * t121609 * t9680;
    let t121615 = t1869 * t117090 * t9089 * t1894;
    let t121619 = 0.20833333333333333334e-1 * t116790 * t9922 + 0.13265555555555555555e-1 * t121586 + 0.33163888888888888888e-2 * t121589 - 0.23148148148148148149e-2 * t112518 + 0.11054629629629629629e-2 * t121592 + 0.13402777777777777778e-2 * t121594 - 0.36848765432098765431e-3 * t112523 + 0.69444444444444444447e-2 * t121597 + 0.10416666666666666667e-1 * t121600 * t9672 + 0.10416666666666666667e-1 * t121600 * t9652 + 0.40208333333333333335e-2 * t121606 * t9652 + 0.1621345679012345679e-1 * t121611 - 0.1492375e-1 * t121615 + 0.20833333333333333334e-1 * t121284 * t9652;
    (t121611, t121615, t121619)
}
