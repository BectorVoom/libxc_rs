//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1072/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1072<F: Float>(t30171: F, t30181: F, t30184: F, t30192: F, t30195: F, t30198: F, t30200: F, t30212: F, t30217: F, t33941: F, t38859: F, t38863: F, t38867: F, t38871: F, t38875: F, t38879: F, t38886: F) -> F {
    let t38888 = F::cast_from(0.94344276868812456204e-3_f64) * t38859 + F::cast_from(0.47172138434406228102e-2_f64) * t38863 - F::cast_from(0.31448092289604152068e-2_f64) * t38867 - F::cast_from(0.18868855373762491241e-2_f64) * t38871 + F::cast_from(0.10482697429868050689e-3_f64) * t38875 - F::cast_from(0.23586069217203114051e-2_f64) * t38879 - t30171 - t30181 + t30184 + t30192 - t30195 - t30198 - t30200 - F::cast_from(0.62896184579208304136e-3_f64) * t30212 - F::cast_from(0.13976929906490734252e-2_f64) * t30217 + F::cast_from(0.53592522647587171215e-3_f64) * t38886 - t33941;
    t38888
}
