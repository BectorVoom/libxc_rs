//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1151/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1151<F: Float>(t31350: F, t4921: F, t30219: F, t8473: F, t4680: F, t7426: F, t8605: F, t30468: F, t4916: F, t31346: F, t4419: F, t15386: F, t31195: F, t35749: F) -> (F, F, F, F, F, F) {
    let t35792 = t31350 * t4921;
    let t35794 = t30219 * t8473;
    let t35795 = F::new(0.47172138434406228102e-2) * t35794;
    let t35797 = t7426 * t4680 * t8605;
    let t35798 = F::new(0.42874018118069736972e-3) * t35797;
    let t35799 = t30468 * t4916;
    let t35800 = F::new(0.34299214494455789578e-2) * t35799;
    let t35801 = t31346 * t4419;
    let t35804 = t31195 * t15386 * t35749;
    (t35792, t35795, t35798, t35800, t35801, t35804)
}
