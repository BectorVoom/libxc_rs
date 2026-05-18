//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 574/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk574<F: Float>(t1659: F, t857: F, t1603: F, t315: F, t323: F, t310: F, t545: F, t464: F, t1410: F, t180: F, t1533: F, t1539: F) -> (F, F, F, F, F, F) {
    let t4130 = F::new(0.13170898365871023197e1) * t857 * t1659;
    let t4131 = t315 * t1603;
    let t4133 = F::new(0.13170898365871023197e1) * t4131 * t323;
    let t4137 = t310 * t545;
    let t4139 = F::new(0.13170898365871023197e1) * t4137 * t464;
    let t4146 = t180 * t1410;
    let t4147 = t4146 * t1533;
    let t4150 = t4146 * t1539;
    (t4130, t4133, t4139, t4146, t4147, t4150)
}
