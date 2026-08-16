//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1136/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1136(t10560: f64, t1775: f64, t10261: f64, t10388: f64, t10575: f64, t10580: f64, t2: f64, t2681: f64, t2682: f64, t2739: f64, t42071: f64, t42075: f64, t42088: f64, t42096: f64, t43843: f64, t43848: f64, t43850: f64, t43852: f64, t43860: f64, t43867: f64, t43872: f64, t43874: f64, t462: f64, t848: f64) -> f64 {
    let t43879 = t1775 * t10560;
    let t43881 = -8.0_f64 * t43843 + 40.0_f64 / 9.0_f64 * t462 * t10580 * t42088 - 8.0_f64 / 9.0_f64 * t43848 - 16.0_f64 / 27.0_f64 * t43850 - 80.0_f64 / 81.0_f64 * t462 * t43852 * t42096 + 8.0_f64 * t462 * t2681 * t10575 * t10388 + 40.0_f64 / 81.0_f64 * t43860 - 36.0_f64 * t462 * t10261 * t2 * t2682 * t2739 + 8.0_f64 / 3.0_f64 * t43867 + 8.0_f64 * t462 * t848 * t42071 + 112.0_f64 / 81.0_f64 * t43872 + 16.0_f64 / 9.0_f64 * t43874 + 2.0_f64 * t462 * t848 * t42075 - 16.0_f64 / 9.0_f64 * t43879;
    t43881
}
