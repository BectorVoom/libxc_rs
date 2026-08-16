//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2055/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2055<F: Float>(t87776: F, t23171: F, t23228: F, t7488: F, t214: F, t4265: F, t25055: F, t81591: F, t25217: F, t6547: F, t25060: F, t225: F, t25222: F) -> (F, F, F, F, F, F, F) {
    let t87777 = F::cast_from(0.82246703342411321824e-2_f64) * t87776;
    let t87779 = t23171 * t23228 * t7488;
    let t87782 = t214 * t4265;
    let t87786 = t81591 * t25055;
    let t87787 = F::cast_from(0.76763589786250567036e-1_f64) * t87786;
    let t87796 = t6547 * t25217;
    let t87797 = F::cast_from(0.38381794893125283518e-1_f64) * t87796;
    let t87804 = t6547 * t25060;
    let t87805 = F::cast_from(0.38381794893125283518e-1_f64) * t87804;
    let t87810 = t25222 * t225;
    (t87777, t87779, t87782, t87787, t87797, t87805, t87810)
}
