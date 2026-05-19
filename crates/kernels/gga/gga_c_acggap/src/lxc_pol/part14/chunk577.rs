//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 577/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk577<F: Float>(t3765: F, t527: F, t1462: F, t997: F, t172: F, t3706: F, t1456: F, t1381: F, t912: F, t2971: F, t2975: F, t484: F, t709: F) -> (F, F, F, F, F, F, F, F) {
    let t4996 = t3765 * t527;
    let t5007 = F::cast_from(0.12004725073059526352e-1_f64) * t997 * t1462;
    let t5011 = t172 * t3706;
    let t5017 = t997 * t1456;
    let t5026 = t1381 * t912;
    let t5028 = F::new(48.0) * t2971;
    let t5030 = F::new(80.0) * t2975;
    let t5032 = t709 * t484;
    (t4996, t5007, t5011, t5017, t5026, t5028, t5030, t5032)
}
