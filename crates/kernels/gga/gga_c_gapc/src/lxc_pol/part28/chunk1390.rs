//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1390/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1390<F: Float>(t34054: F, t34056: F, t34060: F, t34062: F, t34036: F, t36800: F, t36801: F, t36802: F, t36803: F, t36804: F, t36805: F, t34066: F) -> (F, F) {
    let t36806 = F::new(0.28605695478281987903e-5) * t34054;
    let t36807 = F::new(0.14068374825384584215e-7) * t34056;
    let t36808 = F::new(0.46573198186092908864e-9) * t34060;
    let t36809 = F::new(0.49520679385353736436e-5) * t34062;
    let t36810 = -F::new(0.11666621455439814816e-3) * t34036 + t36800 - t36801 - t36802 + t36803 - t36804 + t36805 + t36806 + t36807 + t36808 + t36809;
    let t36812 = F::new(0.67528199161846004232e-6) * t34066;
    (t36810, t36812)
}
