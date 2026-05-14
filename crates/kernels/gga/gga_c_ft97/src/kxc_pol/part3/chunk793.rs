//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 793/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk793<F: Float>(t18070: F, t701: F, t17727: F, t2320: F, t17732: F, t3806: F, t172: F, t228: F, t231: F, t4995: F, t13643: F, t18051: F, t18055: F, t18058: F, t18061: F, t18064: F, t18066: F, t18068: F, t9637: F) -> (F, F, F, F, F) {
    let t18071 = t701 * t18070;
    let t18073 = t2320 * t17727;
    let t18074 = t701 * t18073;
    let t18076 = t3806 * t17732;
    let t18077 = t701 * t18076;
    let t18081 = t228 * t4995 * t172 * t231;
    let t18083 = 0.6384360837962962963e-2 * t18051 + 0.2269994964609053498e-1 * t13643 - 0.51074886703703703704e-1 * t18055 + 0.19862455940329218107e-1 * t18058 - 0.34049924469135802469e-1 * t18061 + 0.38306165027777777778e-1 * t18064 + 0.6809984893827160494e-1 * t18066 - 0.4539989929218106996e-1 * t18068 + 0.51074886703703703704e-1 * t18071 - 0.12768721675925925926e-1 * t18074 + 0.85124811172839506173e-2 * t18077 + t9637 + 0.62424861526748971193e-1 * t18081;
    (t18071, t18074, t18077, t18081, t18083)
}
