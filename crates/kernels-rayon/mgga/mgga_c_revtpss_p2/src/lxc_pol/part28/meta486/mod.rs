//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1847;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1848;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta486(t10416: f64, t1936: f64, t13435: f64, t2322: f64, t7002: f64, t13440: f64, t5523: f64, t112: f64, t239: f64, t624: f64, t655: f64, t665: f64, t114: f64, t2339: f64, t68: f64, t2340: f64, t2366: f64, t6998: f64, t1312: f64, t2371: f64, t25096: f64, t25169: f64, t25805: f64, t670: f64, t6985: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t25812, t25814, t25816, t25818, t25820, t25822, t25823, t25824) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1847(t10416, t1936, t13435, t2322, t7002, t13440, t5523, t112, t239, t624, t655, t665);
        let (t25826, t25832) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1848(t114, t25824, t2339, t68, t2340, t2366, t6998, t25822);
        let t25835 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1849(t1312, t25832, t2371, t25096, t25169, t25805, t25812, t25814, t25816, t25818, t25820, t670, t6985);
    (t25822, t25823, t25824, t25826, t25832, t25835)
}
