//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1027/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1027<F: Float>(t2633: F, t4180: F, t4181: F, t13029: F, t225: F, t237: F, t2697: F, t4261: F, t12971: F, t820: F, t847: F, t9645: F) -> (F, F, F, F, F, F) {
    let t13333 = t4180 * t4181 * t2633;
    let t13336 = t13029 * t225;
    let t13337 = t13336 * t237;
    let t13345 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t2697 * t4261;
    let t13347 = t847 * t820 * t12971;
    let t13350 = t9645 * t820;
    (t13333, t13336, t13337, t13345, t13347, t13350)
}
