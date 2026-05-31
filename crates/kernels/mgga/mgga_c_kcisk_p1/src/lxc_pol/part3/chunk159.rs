//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 159/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk159<F: Float>(t311: F, t312: F, t579: F, t571: F, t574: F, t577: F, t573: F) -> (F, F, F, F, F, F) {
    let t581 = t311 * t312 * t579;
    let t583 = F::cast_from(0.379785e1_f64) * t574 + F::cast_from(0.8969e0_f64) * t571 + F::cast_from(0.204775e0_f64) * t577 + F::cast_from(0.123235e0_f64) * t581;
    let t586 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t583;
    let t587 = F::ln(t586);
    let t589 = F::cast_from(0.62182e-1_f64) * t573 * t587;
    let t591 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t571;
    (t581, t583, t586, t587, t589, t591)
}
