//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 779/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk779(t33835: f64, t7641: f64, t7512: f64, t7638: f64, t2781: f64, t33812: f64, t193: f64, t6308: f64, t1486: f64, t681: f64, t7646: f64, t2: f64, t7584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33836 = t7641 * t33835;
    let t33838 = t7638 * t7512 * t33836;
    let t33840 = t2781 * t33812;
    let t33842 = t6308 * t193 * t33840;
    let t33845 = t1486 * t681 * t7646;
    let t33846 = t33845 / 9.0_f64;
    let t33847 = t2 * t7584;
    (t33836, t33838, t33840, t33842, t33845, t33846, t33847)
}
