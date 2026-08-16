//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2155/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2155<F: Float>(t25217: F, t6547: F, t25060: F, t82209: F, t82211: F, t225: F, t25222: F, t1880: F, t23237: F, t25216: F, t1912: F, t218: F, t23281: F, t259: F, t4273: F, t46508: F, t47618: F, t7517: F, t82219: F, t82221: F, t82230: F, t82236: F, t866: F, t87512: F, t9593: F) -> (F, F, F, F, F) {
    let t87796 = t6547 * t25217;
    let t87797 = F::cast_from(0.38381794893125283518e-1_f64) * t87796;
    let t87804 = t6547 * t25060;
    let t87805 = F::cast_from(0.38381794893125283518e-1_f64) * t87804;
    let t87806 = F::cast_from(0.25587863262083522346e0_f64) * t82209;
    let t87807 = F::cast_from(0.12793931631041761173e0_f64) * t82211;
    let t87810 = t25222 * t225;
    let t87822 = t1880 * t23237 * t25216;
    let t87827 = -t82219 - F::cast_from(2.0_f64) * t87810 * t866 + F::cast_from(0.16449340668482264365e-1_f64) * t82221 + F::cast_from(4.0_f64) * t23281 * t4273 + F::cast_from(4.0_f64) * t9593 * t7517 - F::cast_from(0.38381794893125283518e-1_f64) * t82230 - t46508 * t1912 - t47618 * t1912 - F::cast_from(0.16449340668482264365e-1_f64) * t87822 - F::cast_from(0.41123351671205660912e-2_f64) * t82236 + t218 * t87512 * t259;
    (t87797, t87805, t87806, t87807, t87827)
}
