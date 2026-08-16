//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2204/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2204(t5544: f64, t606: f64, t16662: f64, t25: f64, t2752: f64, t28447: f64, t28248: f64, t776: f64, t22960: f64, t10143: f64, t1408: f64, t25374: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98046 = t606 * t5544;
    let t98050 = t25 * t16662;
    let t98054 = t28447 * t2752;
    let t98058 = t28248 * t776;
    let t98059 = t22960 * t98058;
    let t98064 = t10143 * t1408;
    let t98065 = t98064 * t25374;
    (t98046, t98050, t98054, t98058, t98059, t98065)
}
