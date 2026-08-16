//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1079/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1079(t14879: f64, t2531: f64, t14842: f64, t14845: f64, t14849: f64, t14852: f64, t14856: f64, t14860: f64, t14862: f64, t14865: f64, t14868: f64, t14871: f64, t14874: f64, t14878: f64, t2594: f64, t2619: f64, t8915: f64, t8922: f64) -> (f64, f64) {
    let t14881 = 0.32163958997385070134e2_f64 * t2531 * t14879;
    let t14882 = -0.10389515463408878255e3_f64 * t8915 * t14842 - 0.11696447245269292414e1_f64 * t2594 * t14845 + 0.17315859105681463759e2_f64 * t2619 * t14849 + 0.34631718211362927518e2_f64 * t2619 * t14852 + 0.10254018858216406658e4_f64 * t8922 * t14856 + t14860 - t14862 - t14865 + t14868 + t14871 + t14874 - t14878 - t14881;
    (t14881, t14882)
}
