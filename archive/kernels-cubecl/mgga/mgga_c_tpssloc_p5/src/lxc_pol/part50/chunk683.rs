//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 683/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk683<F: Float>(t240: F, t6619: F, t812: F, t849: F, t6580: F, t6582: F, t6587: F, t6594: F, t6603: F, t6607: F, t6610: F, t6615: F, t6618: F) -> (F, F, F) {
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    let t6622 = t6621 * t849;
    let t6624 = -t6580 - t6582 / F::cast_from(48.0_f64) - t6587 - F::cast_from(0.12111826828242117256e-2_f64) * t6594 - t6603 - F::cast_from(0.20186378047070195427e-3_f64) * t6607 + t6610 / F::cast_from(1536.0_f64) - t6615 / F::cast_from(1536.0_f64) - t6618 - t6622 / F::cast_from(384.0_f64);
    (t6620, t6621, t6624)
}
