//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 696/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk696(t1785: f64, t5030: f64, t5038: f64, t7261: f64, t1636: f64, t5015: f64, t10593: f64, t7242: f64, t1764: f64, t3934: f64, t654: f64, t4989: f64, t5002: f64) -> (f64, f64, f64, f64, f64) {
    let t10843 = t5030 * t1785;
    let t10844 = t10843 * t5038;
    let t10845 = t7261 * t10844;
    let t10848 = t1636 * t5038;
    let t10849 = t5015 * t10848;
    let t10852 = t7242 * t10593;
    let t10856 = t1764 * t654 * t3934;
    let t10863 = t4989 * t5002;
    (t10845, t10849, t10852, t10856, t10863)
}
