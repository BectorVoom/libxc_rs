//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 570/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk570<F: Float>(t119: F, t1603: F, t1308: F, t872: F, t1221: F, t3875: F, t556: F, t1620: F, t857: F, t1658: F, t463: F, t1220: F) -> (F, F, F, F, F) {
    let t4103 = t119 * t1603;
    let t4107 = F::new(0.13170898365871023197e1) * t1308 * t872;
    let t4109 = t3875 * t556 * t1221;
    let t4113 = F::new(0.26341796731742046394e1) * t857 * t1620;
    let t4118 = t1658 * t463;
    let t4119 = t1220 * t4118;
    (t4103, t4107, t4109, t4113, t4119)
}
