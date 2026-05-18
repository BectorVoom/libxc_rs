//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 916/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk916<F: Float>(t2268: F, t2343: F, t41869: F, t12767: F, t6305: F, t1063: F, t3158: F, t8207: F, t2304: F, t34273: F, t39849: F, t12803: F, t29874: F) -> (F, F, F, F, F, F) {
    let t42835 = t2268 * t2343 * t41869;
    let t42838 = F::new(0.56910013271352299198e-1) * t6305 * t12767;
    let t42841 = F::new(0.19918504644973304719e0) * t1063 * t3158 * t8207;
    let t42844 = F::new(0.39837009289946609438e0) * t2268 * t2304 * t34273;
    let t42845 = F::new(0.142275033178380748e-1) * t39849;
    let t42846 = t29874 * t12803;
    (t42835, t42838, t42841, t42844, t42845, t42846)
}
