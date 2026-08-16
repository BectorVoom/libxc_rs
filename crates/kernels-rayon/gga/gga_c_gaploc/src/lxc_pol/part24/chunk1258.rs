//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1258/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1258(t10961: f64, t2197: f64, t10713: f64, t4614: f64, t833: f64, t24364: f64, t955: f64, t11001: f64, t11004: f64, t11113: f64, t1445: f64, t1710: f64, t2043: f64, t2087: f64, t28022: f64, t32748: f64, t32753: f64, t32756: f64, t32759: f64, t32761: f64, t32764: f64, t32766: f64, t32769: f64, t5666: f64) -> f64 {
    let t32771 = 0.30674340763136599742e2_f64 * t2197 * t10961;
    let t32774 = 0.30674340763136599742e2_f64 * t833 * t4614 * t10713;
    let t32778 = 0.79445533226334281487e-1_f64 * t955 * t24364;
    let t32783 = -t32748 - t28022 + 0.51123901271894332905e0_f64 * t5666 * t11113 + t32753 - t32756 - t32759 + t32761 - t32764 - t32766 + t32769 + t32771 + t32774 + 0.35750489951850426669e0_f64 * t2043 * t11001 - t32778 - 0.69017266717057349418e1_f64 * t2087 * t1445 * t11004 * t1710;
    t32783
}
