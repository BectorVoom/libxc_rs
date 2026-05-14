//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1380/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1380<F: Float>(t116413: F, t116903: F, t116994: F, t116997: F, t116999: F, t117008: F, t121765: F, t121767: F, t121770: F, t121772: F, t121774: F, t121777: F, t121787: F, t121789: F, t34154: F, t34218: F, t9922: F) -> (F,) {
    let t121791 = 0.46296296296296296297e-2 * t116994 - t116997 + t116999 + 0.24872916666666666666e-2 * t121765 + 0.22109259259259259259e-2 * t121767 + 0.89351851851851851853e-3 * t121770 + 0.23148148148148148149e-2 * t121772 - 0.58958024691358024689e-2 * t121774 - 0.49745833333333333332e-2 * t121777 + 0.11054629629629629629e-2 * t117008 + 0.8041666666666666667e-2 * t116903 * t9922 + 0.8041666666666666667e-2 * t116413 * t9922 + 0.8041666666666666667e-2 * t34154 * t34218 - 0.11054629629629629629e-2 * t121787 - 0.18518518518518518519e-1 * t121789;
    (t121791,)
}
