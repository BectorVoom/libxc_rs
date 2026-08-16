//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 876/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk876(t10914: f64, t10915: f64, t45320: f64, t45305: f64, t7584: f64, t7585: f64, t10930: f64, t10931: f64, t45423: f64, t11765: f64, t2714: f64, t7572: f64, t7573: f64) -> (f64, f64, f64, f64, f64) {
    let t45633 = 0.42900587942220512002e1_f64 * t10914 * t10915 * t45320;
    let t45636 = 0.11502877786176224903e2_f64 * t7584 * t7585 * t45305;
    let t45639 = 0.27606906686822939767e2_f64 * t10930 * t10931 * t45423;
    let t45648 = 0.35750489951850426669e0_f64 * t2714 * t11765;
    let t45653 = 0.69017266717057349418e1_f64 * t7572 * t7573 * t45305;
    (t45633, t45636, t45639, t45648, t45653)
}
