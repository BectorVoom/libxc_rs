//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 325/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk325<F: Float>(t524: F, t538: F, t398: F, t1553: F) -> (F, F, F, F) {
    let t536 = F::cast_from(0.0_f64) < t524;
    let t1587 = t538 * t538;
    let t1588 = F::cast_from(1.0_f64) / t1587;
    let t1589 = t398 * t1588;
    let t1591 = piecewise3::<F>(t536, t1553, -t1553);
    (t1587, t1588, t1589, t1591)
}
