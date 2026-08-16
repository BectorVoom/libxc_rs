//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1099/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1099(t10683: f64, t24980: f64, t28741: f64, t33847: f64, t2862: f64, t28816: f64, t6318: f64, t28735: f64, t28736: f64, t33868: f64, t4162: f64, t6317: f64) -> (f64, f64, f64, f64) {
    let t152746 = t24980 * t10683 * t33847 * t28741;
    let t152750 = t24980 * t2862 * t6318 * t28816;
    let t152754 = t28735 * t2862 * t33847 * t28736;
    let t152758 = t6317 * t10683 * t33868 * t4162;
    (t152746, t152750, t152754, t152758)
}
