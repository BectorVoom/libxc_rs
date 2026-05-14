//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 583/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk583<F: Float>(t1676: F, t1685: F, t8607: F, t4787: F, t8590: F, t4790: F, t1674: F, t2396: F, t45: F, t6851: F, t8546: F, t8548: F, t8552: F, t8576: F, t8579: F, t8585: F, t8592: F) -> (F, F, F) {
    let t8609 = t1676 * t8607 * t1685;
    let t8612 = t4787 * t8590;
    let t8613 = t8612 * t4790;
    let t8616 = -t8546 + t8548 - t8552 + t8576 + t8579 + 0.19751789702565206229e-1 * t45 * t8585 - 0.11696446794910408142e1 * t6851 * t2396 + 0.11696446794910408142e1 * t1674 * t8592 - 0.58482233974552040708e0 * t1674 * t8609 - 0.17315755899375863299e2 * t1674 * t8613;
    (t8609, t8613, t8616)
}
