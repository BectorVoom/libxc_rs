//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1072/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1072(t30171: f64, t30181: f64, t30184: f64, t30192: f64, t30195: f64, t30198: f64, t30200: f64, t30212: f64, t30217: f64, t33941: f64, t38859: f64, t38863: f64, t38867: f64, t38871: f64, t38875: f64, t38879: f64, t38886: f64) -> f64 {
    let t38888 = 0.94344276868812456204e-3_f64 * t38859 + 0.47172138434406228102e-2_f64 * t38863 - 0.31448092289604152068e-2_f64 * t38867 - 0.18868855373762491241e-2_f64 * t38871 + 0.10482697429868050689e-3_f64 * t38875 - 0.23586069217203114051e-2_f64 * t38879 - t30171 - t30181 + t30184 + t30192 - t30195 - t30198 - t30200 - 0.62896184579208304136e-3_f64 * t30212 - 0.13976929906490734252e-2_f64 * t30217 + 0.53592522647587171215e-3_f64 * t38886 - t33941;
    t38888
}
