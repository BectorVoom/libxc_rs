//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1044/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1044(t38872: f64, t38881: f64, t38886: f64, t34803: f64, t34810: f64, t34820: f64, t37228: f64, t38874: f64, t38876: f64, t38889: f64, t38899: f64, t38901: f64, t38908: f64, t38913: f64, t38918: f64, t38922: f64, t38926: f64) -> f64 {
    let t42767 = 0.20496175532535769482e-3_f64 * t38872;
    let t42771 = 0.86737941314158990616e-4_f64 * t38881;
    let t42772 = 0.86737941314158990616e-4_f64 * t38886;
    let t42783 = -t42767 - 0.3842256877732895568e-2_f64 * t38874 + 0.92232789896410962669e-3_f64 * t38876 - 0.53337116123857557162e0_f64 * t34803 + t42771 + t42772 + 0.162600798888400151e-2_f64 * t38889 + 0.20455996240684006298e-1_f64 * t38899 - t37228 - 0.2727466165424534173e-1_f64 * t38901 - 0.1333427903096438929e0_f64 * t34810 - 0.36366215538993788974e-1_f64 * t34820 - 0.638468998399467591e-4_f64 * t38908 - 0.638468998399467591e-4_f64 * t38913 - 0.5107751987195740728e-4_f64 * t38918 + 0.15323255961587222184e-3_f64 * t38922 - 0.10215503974391481456e-3_f64 * t38926;
    t42783
}
