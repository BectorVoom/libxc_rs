//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 932/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk932<F: Float>(t13176: F, t816: F, t1512: F, t9671: F, t2697: F, t4257: F, t2563: F, t4159: F, t4155: F, t9573: F, t2644: F, t820: F) -> (F, F, F, F, F, F) {
    let t13177 = t13176 * t816;
    let t13182 = t9671 * t1512;
    let t13190 = F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t2697 * t4257;
    let t13202 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2563 * t4159;
    let t13208 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9573 * t4155;
    let t13222 = t2644 * t820;
    (t13177, t13182, t13190, t13202, t13208, t13222)
}
