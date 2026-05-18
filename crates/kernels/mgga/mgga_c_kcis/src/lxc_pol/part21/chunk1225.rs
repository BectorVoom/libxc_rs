//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1225/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1225<F: Float>(t7609: F, t9312: F, t26514: F, t898: F, t26419: F, t8522: F, t2146: F, t9274: F, t9276: F, t2165: F, t26556: F, t26634: F, t2766: F, t2771: F, t2789: F, t7660: F, t7669: F, t9010: F, t9017: F, t906: F, t9185: F, t92351: F, t92356: F) -> (F, F, F, F) {
    let t92360 = t7609 * t9312;
    let t92364 = t26514 * t898;
    let t92368 = F::new(12.0) * t8522 * t26419;
    let t92373 = t2146 * t9274;
    let t92375 = F::new(6.0) * t92373 * t9276;
    let t92376 = F::new(2.0) * t2165 * t2771 * t9185 + F::new(6.0) * t2771 * t2789 * t7669 - F::new(18.0) * t2789 * t7660 * t9017 - F::new(3.0) * t26556 * t2766 + F::new(6.0) * t26634 * t9010 - F::new(3.0) * t906 * t92364 + t92351 - t92356 + t92360 - t92368 + t92375;
    (t92360, t92368, t92375, t92376)
}
