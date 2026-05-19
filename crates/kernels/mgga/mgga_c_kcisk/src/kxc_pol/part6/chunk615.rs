//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 615/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk615<F: Float>(t4663: F, t8504: F, t1646: F, t8522: F, t4676: F, t6756: F, t8512: F, t8516: F, t8520: F, t1815: F, t2372: F, t4664: F, t574: F, t6774: F) -> (F, F, F, F) {
    let t8525 = t4663 * t8504;
    let t8527 = t1646 * t8522;
    let t8533 = -F::new(0.991e-2) * t8525 + F::new(0.1982e-1) * t8527 + t4676 + F::cast_from(0.27516666666666666666e-2_f64) * t6756 - F::cast_from(0.27516666666666666667e-2_f64) * t8512 + F::new(0.8255e-2) * t8516 - F::new(0.41275e-2) * t8520;
    let t8536 = -t4664 * t8504 / F::new(8.0) + t6774 * t2372 / F::new(2.0) + t1815 * t8522 / F::new(4.0) + t574 * t8533 / F::new(2.0);
    (t8525, t8527, t8533, t8536)
}
