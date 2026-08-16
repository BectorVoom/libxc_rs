//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 865/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk865(t35833: f64, t7641: f64, t7512: f64, t7638: f64, t2781: f64, t35819: f64, t193: f64, t6308: f64, t10248: f64, t1091: f64, t33847: f64, t6317: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35834 = t7641 * t35833;
    let t35836 = t7638 * t7512 * t35834;
    let t35838 = t2781 * t35819;
    let t35840 = t6308 * t193 * t35838;
    let t35843 = t10248 * t33847 * t1091;
    let t35844 = t6317 * t35843;
    (t35834, t35836, t35838, t35840, t35843, t35844)
}
