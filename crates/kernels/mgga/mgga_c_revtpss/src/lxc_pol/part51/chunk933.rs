//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 933/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk933<F: Float>(t1096: F, t8507: F, t31959: F, t1052: F, t359: F, t369: F, t8499: F, t11921: F, t247: F, t385: F, t8502: F, t1982: F, t3140: F) -> (F, F, F, F, F, F, F) {
    let t31960 = t8507 * t1096;
    let t31961 = t31959 * t31960;
    let t31964 = t359 * t1052;
    let t31965 = t31964 * t369;
    let t31966 = t8499 * t31965;
    let t31970 = t247 * t11921 * t385;
    let t31972 = F::new(0.18822977838986977999e-3) * t8502 * t31970;
    let t31973 = t1982 * t3140;
    (t31961, t31964, t31965, t31966, t31970, t31972, t31973)
}
