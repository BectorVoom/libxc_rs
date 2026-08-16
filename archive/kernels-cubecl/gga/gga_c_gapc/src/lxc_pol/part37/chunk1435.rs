//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1435/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1435<F: Float>(t33969: F, t36773: F, t36774: F, t36775: F, t36777: F, t36778: F, t36779: F, t36780: F, t36781: F, t36782: F, t36783: F, t34036: F, t36800: F, t36801: F, t36802: F, t36803: F, t36804: F, t36805: F, t36806: F, t36807: F, t36808: F, t36809: F) -> (F, F) {
    let t38779 = -t36773 - t36774 + t36775 - F::cast_from(0.25301106770833333334e-5_f64) * t33969 + t36777 - t36778 + t36779 + t36780 - t36781 - t36782 - t36783;
    let t38788 = -F::cast_from(0.11666621455439814815e-3_f64) * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    (t38779, t38788)
}
