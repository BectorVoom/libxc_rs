//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 911/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk911<F: Float>(t13643: F, t18051: F, t18055: F, t18058: F, t18061: F, t18064: F, t18066: F, t18068: F, t18071: F, t18074: F, t18077: F, t18081: F, t9637: F) -> F {
    let t18083 = F::new(0.6384360837962962963e-2) * t18051 + F::new(0.2269994964609053498e-1) * t13643 - F::new(0.51074886703703703704e-1) * t18055 + F::new(0.19862455940329218107e-1) * t18058 - F::new(0.34049924469135802469e-1) * t18061 + F::new(0.38306165027777777778e-1) * t18064 + F::new(0.6809984893827160494e-1) * t18066 - F::new(0.4539989929218106996e-1) * t18068 + F::new(0.51074886703703703704e-1) * t18071 - F::new(0.12768721675925925926e-1) * t18074 + F::new(0.85124811172839506173e-2) * t18077 + t9637 + F::new(0.62424861526748971193e-1) * t18081;
    t18083
}
