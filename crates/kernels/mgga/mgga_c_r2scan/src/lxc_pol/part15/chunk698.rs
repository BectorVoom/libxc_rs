//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 698/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk698<F: Float>(t182: F, t518: F, t190: F, t625: F, t1696: F, t750: F, t1827: F, t732: F, t1842: F, t1810: F, t1838: F, t1826: F, t1830: F) -> (F, F, F, F, F, F, F) {
    let t5332 = t518 * t182;
    let t5335 = F::new(0.55403703703703703703e-1) * t625 * t5332 * t190;
    let t5336 = t1696 * t750;
    let t5338 = t732 * t1827;
    let t5340 = t732 * t1842;
    let t5344 = t732 * t1810;
    let t5346 = t732 * t1838;
    let t5348 = t1826 * t1830;
    (t5335, t5336, t5338, t5340, t5344, t5346, t5348)
}
