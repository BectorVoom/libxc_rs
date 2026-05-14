//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1203/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1203<F: Float>(t2737: F, t2740: F, t32377: F, t32390: F, t32439: F, t33794: F, t33805: F, t33941: F, t34702: F, t34931: F, t34936: F, t34941: F, t34945: F, t34950: F, t34955: F, t9536: F, t9864: F) -> (F,) {
    let t34963 = -0.34722222222222222222e-2 * t33794 * t9864 - 0.34722222222222222222e-2 * t33941 * t9864 - 0.40208333333333333334e-2 * t32439 * t34931 - 0.10416666666666666667e-1 * t2737 * t34936 - 0.52083333333333333333e-2 * t34941 * t2740 - 0.116403125e-2 * t32377 * t34945 + t32390 + 0.19345601851851851852e-2 * t34702 + 0.34722222222222222222e-2 * t9536 * t34950 + 0.34722222222222222222e-2 * t9536 * t34955 + 0.13402777777777777778e-2 * t32439 * t34955 - 0.34722222222222222222e-2 * t33805 - 0.10416666666666666667e-1 * t2737 * t34945;
    (t34963,)
}
