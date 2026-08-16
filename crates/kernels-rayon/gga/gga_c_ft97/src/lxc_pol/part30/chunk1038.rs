//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1038/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1038(t2427: f64, t6793: f64, t224: f64, t2344: f64, t7205: f64, t123619: f64, t5009: f64, t694: f64, t109216: f64, t3766: f64, t6: f64, t150319: f64, t420: f64) -> (f64, f64, f64, f64, f64) {
    let t150843 = t2427 * t6793;
    let t150844 = t224 * t150843;
    let t150845 = t7205 * t2344;
    let t150846 = t150845 * t123619;
    let t150849 = t694 * t5009;
    let t150858 = t3766 * t109216 * t6;
    let t150864 = t150319 * t420;
    (t150844, t150846, t150849, t150858, t150864)
}
