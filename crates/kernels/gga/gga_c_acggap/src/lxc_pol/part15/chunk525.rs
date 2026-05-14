//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 525/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk525<F: Float>(t1620: F, t857: F, t1605: F, t310: F, t1659: F, t1603: F, t315: F, t323: F, t545: F, t464: F, t1410: F, t180: F, t1539: F, t1160: F, t157: F, t879: F) -> (F, F, F, F, F, F, F, F) {
    let t4113 = 0.26341796731742046394e1 * t857 * t1620;
    let t4123 = 0.13170898365871023197e1 * t310 * t1605;
    let t4130 = 0.13170898365871023197e1 * t857 * t1659;
    let t4131 = t315 * t1603;
    let t4133 = 0.13170898365871023197e1 * t4131 * t323;
    let t4137 = t310 * t545;
    let t4139 = 0.13170898365871023197e1 * t4137 * t464;
    let t4146 = t180 * t1410;
    let t4150 = t4146 * t1539;
    let t4152 = 0.13170898365871023197e1 * t1160 * t4150;
    let t4162 = t157 * t879;
    (t4113, t4123, t4130, t4133, t4137, t4139, t4152, t4162)
}
