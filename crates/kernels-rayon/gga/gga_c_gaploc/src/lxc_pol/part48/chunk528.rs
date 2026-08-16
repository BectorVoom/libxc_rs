//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 528/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk528(t3327: f64, t6313: f64, t3338: f64, t599: f64, t475: f64, t2343: f64, t555: f64, t494: f64, t2317: f64, t2761: f64, t6525: f64, t2321: f64, t8237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10150 = 0.37940008847568199465e-1_f64 * t6313 * t3327;
    let t10151 = t599 * t3338;
    let t10152 = t10151 * t475;
    let t10153 = t2343 * t10152;
    let t10156 = t555 * t3338;
    let t10157 = t10156 * t494;
    let t10160 = t2761 * t2317;
    let t10161 = t6525 * t10160;
    let t10162 = 0.11856252764865062333e-2_f64 * t10161;
    let t10163 = t8237 * t2321;
    (t10150, t10151, t10152, t10153, t10156, t10157, t10161, t10162, t10163)
}
