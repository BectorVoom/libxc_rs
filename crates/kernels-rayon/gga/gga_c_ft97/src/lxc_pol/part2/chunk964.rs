//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 964/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk964(t13352: f64, t4199: f64, t10603: f64, t14671: f64, t13346: f64, t4206: f64, t14648: f64, t2771: f64, t14628: f64, t13296: f64, t13301: f64, t14664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14971 = t4199 * t13352;
    let t14974 = t10603 * t14671;
    let t14977 = t4206 * t13346;
    let t14980 = t2771 * t14648;
    let t14983 = t2771 * t14628;
    let t14986 = t4206 * t13296;
    let t14989 = t4206 * t13301;
    let t14992 = t2771 * t14664;
    (t14971, t14974, t14977, t14980, t14983, t14986, t14989, t14992)
}
