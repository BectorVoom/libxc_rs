//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 851/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk851<F: Float>(t2743: F, t5326: F, t1419: F, t959: F, t1422: F, t2483: F, t725: F, t41: F, t2794: F, t410: F, t406: F, t5331: F, t5335: F, t5336: F, t5338: F, t5340: F) -> F {
    let t7699 = t2743 * t5326;
    let t7701 = t1419 * t959;
    let t7703 = t1422 * t959;
    let t7705 = t2483 * t725;
    let t7707 = F::new(2.0) * t41 * t7705;
    let t7708 = t410 * t2794;
    let t7710 = t406 * t2794;
    let t7715 = F::cast_from(0.1350520664e0_f64) * t7699 - F::new(12.0) * t7701 + F::new(32.0) * t7703 - t7707 + F::new(8.0) * t7708 - F::new(8.0) * t7710 - t5331 + t5335 + F::cast_from(0.17315859105681463759e2_f64) * t5336 - F::cast_from(0.46785788981077169656e1_f64) * t5338 + F::cast_from(0.69263436422725855036e2_f64) * t5340;
    t7715
}
