//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1314/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1314<F: Float>(t106020: F, t106024: F, t106026: F, t119828: F, t119832: F, t119837: F, t119842: F, t119847: F, t119850: F, t119853: F, t119856: F, t119860: F, t119868: F, t119872: F, t119876: F, t119879: F, t119881: F, t119886: F, t119889: F, t119892: F, t119895: F, t119898: F, t119902: F) -> (F, F) {
    let t120972 = -t119828 / 6.0 + t119832 / 9.0 + t119837 / 9.0 + t119842 / 9.0 - t119847 / 27.0 + t106020 - 2.0 / 9.0 * t119850 - 2.0 / 9.0 * t119853 - 2.0 / 9.0 * t119856 - 4.0 / 9.0 * t119860 + t106024 - t106026;
    let t120987 = -2.0 / 9.0 * t119868 + t119872 / 9.0 + 8.0 * t119876 + t119879 / 3.0 - t119881 / 18.0 + t119886 / 6.0 - 4.0 / 27.0 * t119889 + 2.0 / 27.0 * t119892 - 2.0 / 9.0 * t119895 - 2.0 / 9.0 * t119898 - t119902 / 9.0;
    (t120972, t120987)
}
