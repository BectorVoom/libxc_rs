//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 862/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk862<F: Float>(t26: F, t28422: F, t1659: F, t28381: F, t28373: F, t4726: F, t1653: F, t28393: F, t10739: F, t28371: F, t28375: F, t28383: F, t28391: F, t28410: F, t28412: F, t28415: F, t28417: F, t28420: F) -> (F, F, F, F, F) {
    let t28423 = t26 * t28422;
    let t28425 = t1659 * t28381;
    let t28426 = t26 * t28425;
    let t28430 = t4726 * t28373;
    let t28431 = t26 * t28430;
    let t28435 = t1653 * t28393;
    let t28437 = -F::new(0.82156666666666666668e-1) * t28410 - F::new(0.28483875e1) * t28412 - t10739 - F::new(0.76790625e-1) * t28415 + F::new(0.142419375e1) * t28417 - F::new(0.36514074074074074075e-1) * t28420 - F::new(0.82156666666666666667e-1) * t28423 - F::new(0.49293999999999999999e0) * t28426 + F::new(0.11958666666666666667e1) * t28375 - F::new(0.17938e1) * t28383 + F::new(0.16431333333333333333e0) * t28431 - F::new(0.33218518518518518518e0) * t28371 - F::new(0.29896666666666666667e0) * t28391 + F::new(0.3071625e0) * t28435;
    (t28423, t28426, t28431, t28435, t28437)
}
