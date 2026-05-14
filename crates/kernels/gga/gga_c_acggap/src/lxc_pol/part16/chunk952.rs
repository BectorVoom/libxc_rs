//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 952/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk952<F: Float>(t1165: F, t6218: F, t7351: F, t7575: F, t1844: F, t301: F, t1181: F, t599: F, t7337: F, t30171: F, t30181: F, t30184: F, t30192: F, t30195: F, t30198: F, t30200: F, t30212: F, t30217: F, t33941: F, t38859: F, t38863: F, t38867: F, t38871: F, t38875: F) -> (F, F) {
    let t38879 = t7575 * t1165 * t7351 * t6218;
    let t38883 = t1844 * t301;
    let t38886 = t7337 * t1181 * t599 * t38883;
    let t38888 = 0.94344276868812456204e-3 * t38859 + 0.47172138434406228102e-2 * t38863 - 0.31448092289604152068e-2 * t38867 - 0.18868855373762491241e-2 * t38871 + 0.10482697429868050689e-3 * t38875 - 0.23586069217203114051e-2 * t38879 - t30171 - t30181 + t30184 + t30192 - t30195 - t30198 - t30200 - 0.62896184579208304136e-3 * t30212 - 0.13976929906490734252e-2 * t30217 + 0.53592522647587171215e-3 * t38886 - t33941;
    (t38883, t38888)
}
