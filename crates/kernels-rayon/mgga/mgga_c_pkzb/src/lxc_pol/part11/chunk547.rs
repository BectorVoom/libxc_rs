//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 547/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk547(t1066: f64, t779: f64, t655: f64, t2888: f64, t154: f64, t2739: f64, t742: f64, t178: f64, t2024: f64, t2020: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2889 = t779 * t1066;
    let t2890 = t2889 * t655;
    let t2891 = t2888 * t2890;
    let t2895 = t154 * t742 * t2739;
    let t2898 = t2024 * t178;
    let t2899 = t2020 * t2898;
    (t2889, t2890, t2891, t2895, t2898, t2899)
}
