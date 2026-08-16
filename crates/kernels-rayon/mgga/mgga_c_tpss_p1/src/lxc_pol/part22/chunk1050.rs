//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1050/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1050(t11361: f64, t11377: f64, t11413: f64, t11431: f64, t294: f64, t11224: f64, t11226: f64, t11228: f64, t11230: f64, t11234: f64, t11237: f64, t11240: f64, t11242: f64, t11245: f64, t11248: f64, t11251: f64, t11255: f64, t11258: f64, t11262: f64, t11265: f64, t11267: f64, t11269: f64) -> (f64, f64) {
    let t11434 = t294 * (t11361 + t11377 + t11413 + t11431);
    let t11435 = -t11224 + t11226 - t11228 - t11230 - t11234 - t11237 - t11240 + t11242 - t11245 - t11248 - t11251 + t11255 + t11258 + t11262 + t11265 + t11267 - t11269 + t11434;
    (t11434, t11435)
}
