//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1265/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1265(t22826: f64, t3009: f64, t590: f64, t7068: f64, t23516: f64, t32616: f64, t28072: f64, t28075: f64, t28079: f64, t28081: f64, t28085: f64, t28089: f64, t32839: f64, t32843: f64, t32846: f64, t32850: f64, t32853: f64, t32856: f64, t32860: f64, t32866: f64) -> f64 {
    let t32870 = 0.30674340763136599742e1_f64 * t22826 * t3009 * t7068 * t590;
    let t32872 = 0.51123901271894332902e1_f64 * t23516 * t32616;
    let t32873 = -t32839 - t32843 - t32846 - t28072 - t28075 - t28079 - t28081 + t28085 - t32850 - t28089 - t32853 + t32856 + t32860 - t32866 - t32870 + t32872;
    t32873
}
