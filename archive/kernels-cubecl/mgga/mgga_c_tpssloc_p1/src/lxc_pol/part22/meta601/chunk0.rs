//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2123/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123<F: Float>(t10523: F, t1573: F, t10629: F, t48096: F, t47730: F, t48155: F, t1556: F, t2842: F, t10828: F, t1580: F, t2841: F, t4351: F) -> (F, F, F, F, F, F, F, F) {
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49139 = F::cast_from(0.27595e0_f64) * t48096;
    let t49144 = F::cast_from(0.40256666666666666668e0_f64) * t47730;
    let t49200 = F::cast_from(0.5519e0_f64) * t48155;
    let t49226 = t2842 * t1556;
    let t49263 = t10828 * t1580;
    let t49269 = t4351 * t2841;
    (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269)
}
