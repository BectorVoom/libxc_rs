//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1367/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1367<F: Float>(t2856: F, t4571: F, t2849: F, t4619: F, t10069: F, t10076: F, t1148: F, t11668: F, t11680: F, t11766: F, t11769: F, t1520: F, t24636: F, t27890: F, t2839: F, t2869: F, t2873: F, t28787: F, t2943: F, t2947: F, t32767: F, t32770: F, t32773: F, t32784: F, t4530: F, t4534: F, t4581: F, t511: F, t529: F, t7854: F, t7913: F, t9935: F, t9974: F) -> (F, F) {
    let t33744 = t4571 * t2856;
    let t33751 = t4619 * t2849;
    let t33761 = 0.576e0 * t7854 * t32784 + 0.1512e1 * t10069 * t32767 + 0.576e0 * t7854 * t32770 - 0.672e0 * t10076 * t32773 + 6.0 * t511 * t11680 * t2873 + 4.0 * t1520 * t9935 + 252.0 * t1148 * t4581 * t2869 - 336.0 * t529 * t11668 * t2873 + 0.36e-1 * t33744 * t2947 + 0.58666666666666666667e-1 * t11766 * t2943 - 3200.0 / 81.0 * t9974 * t11769 - 96.0 * t27890 * t33751 - 1440.0 * t28787 * t4619 * t2839 + 0.58666666666666666666e-1 * t7913 * t4534 + 0.36e-1 * t24636 * t4530;
    (t33751, t33761)
}
