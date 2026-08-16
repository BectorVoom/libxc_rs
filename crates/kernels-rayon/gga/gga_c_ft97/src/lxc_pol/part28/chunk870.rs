//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 870/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk870(t34817: f64, t9236: f64, t1369: f64, t28: f64, t2112: f64, t34822: f64, t1039: f64, t7339: f64, t586: f64, t5890: f64, t32896: f64, t32923: f64, t34811: f64, t34815: f64, t34820: f64, t34825: f64, t34829: f64, t34833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34835 = t9236 * t34817;
    let t34837 = t1369 * t28 * t34835;
    let t34839 = t2112 * t34822;
    let t34841 = t1369 * t28 * t34839;
    let t34843 = t7339 * t1039;
    let t34844 = t586 * t34843;
    let t34846 = t5890 * t28 * t34844;
    let t34848 = 3.0_f64 / 2.0_f64 * t34811 + t32896 + 2.0_f64 / 3.0_f64 * t34815 + 4.0_f64 * t34820 - 2.0_f64 * t34825 - t34829 / 2.0_f64 - t32923 - t34833 / 3.0_f64 - 3.0_f64 * t34837 + 2.0_f64 * t34841 + t34846 / 4.0_f64;
    (t34835, t34837, t34839, t34841, t34843, t34844, t34846, t34848)
}
