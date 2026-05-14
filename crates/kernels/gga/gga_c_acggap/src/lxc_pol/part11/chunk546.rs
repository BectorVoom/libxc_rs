//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 546/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk546<F: Float>(t119: F, t1603: F, t1308: F, t872: F, t1221: F, t3875: F, t556: F, t1620: F, t857: F, t1658: F, t463: F, t1220: F, t1605: F, t310: F, t1215: F, t1265: F, t1608: F, t3856: F, t3859: F, t3862: F, t3869: F, t3871: F, t446: F, t464: F) -> (F, F, F) {
    let t4103 = t119 * t1603;
    let t4107 = 0.13170898365871023197e1 * t1308 * t872;
    let t4109 = t3875 * t556 * t1221;
    let t4113 = 0.26341796731742046394e1 * t857 * t1620;
    let t4118 = t1658 * t463;
    let t4119 = t1220 * t4118;
    let t4123 = 0.13170898365871023197e1 * t310 * t1605;
    let t4128 = -0.13170898365871023197e1 * t4103 * t464 + t4107 - 0.39512695097613069591e1 * t446 * t4109 + t4113 - 0.65854491829355115987e0 * t1608 * t1265 - 0.65854491829355115987e0 * t3856 - 0.65854491829355115987e0 * t3859 + 0.26341796731742046394e1 * t446 * t4119 + t3862 + t4123 + 0.26341796731742046394e1 * t3869 + 0.26341796731742046394e1 * t1215 * t1620 + 0.65854491829355115987e0 * t3871;
    (t4109, t4119, t4128)
}
