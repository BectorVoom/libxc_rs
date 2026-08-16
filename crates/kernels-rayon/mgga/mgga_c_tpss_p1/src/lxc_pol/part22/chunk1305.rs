//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1305/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1305(t1275: f64, t5960: f64, t19010: f64, t550: f64, t116: f64, t18627: f64, t1856: f64, t3398: f64, t1848: f64, t3413: f64, t1284: f64, t5941: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63114 = t1275 * t5960;
    let t63116 = t19010 * t550;
    let t63152 = t116 * t18627;
    let t63167 = t3398 * t1856;
    let t63169 = t1848 * t3413;
    let t63173 = t5941 * t1284;
    (t63114, t63116, t63152, t63167, t63169, t63173)
}
