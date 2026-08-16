//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 983/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk983(t1872: f64, t5394: f64, t1291: f64, t6879: f64, t19847: f64, t19850: f64, t19852: f64, t19854: f64, t19858: f64, t19860: f64, t19863: f64, t19866: f64, t19868: f64, t19871: f64, t19873: f64, t19875: f64, t19877: f64, t19880: f64, t19883: f64, t19886: f64, t19888: f64, t19892: f64) -> (f64, f64, f64) {
    let t20724 = t1872 * t5394;
    let t20728 = t6879 * t1291;
    let t20749 = -0.89930555555555555553e-2_f64 * t19847 + 0.26979166666666666666e-1_f64 * t19850 + 0.53958333333333333333e-1_f64 * t19852 + 0.33333333333333333333e0_f64 * t19854 - 0.53958333333333333332e-1_f64 * t19858 - 0.125e0_f64 * t19860 + 0.71944444444444444443e-1_f64 * t19863 - 0.20234375e-1_f64 * t19866 + 0.625e-1_f64 * t19868 - 0.625e-1_f64 * t19871 - 0.125e0_f64 * t19873 + 0.5e0_f64 * t19875 + 0.26979166666666666666e-1_f64 * t19877 - 0.20833333333333333333e-1_f64 * t19880 + 0.60703125e-1_f64 * t19883 + 0.10791666666666666667e0_f64 * t19886 - 0.25e0_f64 * t19888 + 0.41666666666666666667e-1_f64 * t19892;
    (t20724, t20728, t20749)
}
