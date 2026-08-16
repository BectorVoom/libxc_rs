//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1847;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1848;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta486<F: Float>(t10416: F, t1936: F, t13435: F, t2322: F, t7002: F, t13440: F, t5523: F, t112: F, t239: F, t624: F, t655: F, t665: F, t114: F, t2339: F, t68: F, t2340: F, t2366: F, t6998: F, t1312: F, t2371: F, t25096: F, t25169: F, t25805: F, t670: F, t6985: F) -> (F, F, F, F, F, F) {
        let (t25812, t25814, t25816, t25818, t25820, t25822, t25823, t25824) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1847::<F>(t10416, t1936, t13435, t2322, t7002, t13440, t5523, t112, t239, t624, t655, t665);
        let (t25826, t25832) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1848::<F>(t114, t25824, t2339, t68, t2340, t2366, t6998, t25822);
        let t25835 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1849::<F>(t1312, t25832, t2371, t25096, t25169, t25805, t25812, t25814, t25816, t25818, t25820, t670, t6985);
    (t25822, t25823, t25824, t25826, t25832, t25835)
}
