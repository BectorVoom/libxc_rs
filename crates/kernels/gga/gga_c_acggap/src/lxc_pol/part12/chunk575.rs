//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 575/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk575<F: Float>(t1529: F, t310: F, t1633: F, t157: F, t864: F, t1629: F, t3088: F, t1642: F, t3378: F, t1539: F, t4166: F, t1160: F) -> (F, F, F, F, F, F) {
    let t4180 = t310 * t1529;
    let t4182 = F::cast_from(0.26341796731742046394e1_f64) * t4180 * t1633;
    let t4183 = t157 * t864;
    let t4184 = t1629 * t4183;
    let t4185 = t3088 * t4184;
    let t4188 = F::cast_from(0.13170898365871023197e1_f64) * t3378 * t1642;
    let t4189 = t4166 * t1539;
    let t4191 = F::cast_from(0.13170898365871023197e1_f64) * t1160 * t4189;
    (t4180, t4182, t4183, t4185, t4188, t4191)
}
