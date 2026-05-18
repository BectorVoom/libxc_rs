//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1194/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1194<F: Float>(t34995: F, t3801: F, t125070: F, t125074: F, t125092: F, t1298: F, t1300: F, t131426: F, t131474: F, t131512: F, t131552: F, t131599: F, t131640: F, t131686: F, t131725: F, t131771: F, t131815: F, t131849: F, t131882: F, t131925: F, t131966: F, t132005: F, t132047: F, t1832: F, t198: F, t27037: F, t27041: F, t29313: F, t29322: F, t33533: F, t33539: F, t336: F, t5023: F, t5501: F, t7669: F, t7673: F, t8220: F) -> F {
    let t132055 = t34995 * t3801;
    let t132085 = t198 * t336 * (t131426 + t131474 + t131512 + t131552 + t131599 + t131640 + t131686 + t131725 + t131771 + t131815 + t131849 + t131882 + t131925 + t131966 + t132005 + t132047) * t1300 - t5023 * t132055 * t1298 - t5023 * t125070 * t1832 + F::new(2.0) * t5023 * t125074 * t29322 - t5023 * t33533 * t5501 - F::new(2.0) * t5023 * t27037 * t8220 + F::new(4.0) * t5023 * t27041 * t8220 * t1298 - F::new(2.0) * t5023 * t7673 * t29313 + F::new(4.0) * t5023 * t27041 * t1832 * t7669 - F::new(6.0) * t5023 * t125092 * t29322 + F::new(2.0) * t5023 * t33539 * t5501;
    t132085
}
