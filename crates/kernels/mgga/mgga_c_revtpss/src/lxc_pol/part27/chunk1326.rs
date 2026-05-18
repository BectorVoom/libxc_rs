//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1326/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1326<F: Float>(t265: F, t502: F, t27033: F, t3801: F, t12587: F, t7669: F, t2155: F, t44126: F, t12584: F, t1298: F, t1300: F, t13190: F, t198: F, t27037: F, t27041: F, t336: F, t3794: F, t3798: F, t5023: F, t60126: F, t7673: F, t94213: F, t96913: F, t96964: F, t97015: F, t97072: F, t97323: F, t97375: F, t97428: F, t97480: F) -> F {
    let t503 = t265 < t502;
    let t97487 = t27033 * t3801;
    let t97491 = t7669 * t12587;
    let t97498 = t2155 * t44126;
    let t97508 = piecewise3::<f64>(t503, t198 * t336 * (t96913 + t96964 + t97015 + t97072 + t97323 + t97375 + t97428 + t97480) * t1300 - F::new(3.0) * t5023 * t97487 * t1298 + F::new(6.0) * t5023 * t97491 * t3798 - F::new(3.0) * t5023 * t27037 * t3794 - F::new(6.0) * t5023 * t97498 * t12584 + F::new(6.0) * t5023 * t27041 * t60126 - t5023 * t7673 * t13190, t94213);
    t97508
}
