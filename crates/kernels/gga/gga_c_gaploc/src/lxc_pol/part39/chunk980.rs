//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 980/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk980<F: Float>(t1063: F, t2854: F, t29969: F, t6320: F, t12767: F, t6313: F, t2268: F, t2756: F, t3152: F, t39866: F, t39869: F, t39893: F) -> (F, F, F, F, F, F) {
    let t42857 = F::cast_from(0.17073003981405689759e0_f64) * t1063 * t6320 * t2854 * t29969;
    let t42863 = F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t12767;
    let t42866 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t3152 * t2756;
    let t42867 = F::cast_from(0.47425011059460249332e-2_f64) * t39866;
    let t42868 = F::cast_from(0.94850022118920498664e-2_f64) * t39869;
    let t42869 = F::cast_from(0.71137516589190373998e-2_f64) * t39893;
    (t42857, t42863, t42866, t42867, t42868, t42869)
}
