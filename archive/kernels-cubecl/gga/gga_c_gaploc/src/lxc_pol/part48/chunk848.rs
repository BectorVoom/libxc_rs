//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 848/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk848<F: Float>(t10215: F, t2787: F, t11264: F, t2268: F, t6949: F, t13277: F, t6305: F, t13268: F, t13307: F, t6313: F, t42846: F, t42849: F) -> (F, F, F, F, F, F, F) {
    let t44564 = t2787 * t10215;
    let t44572 = F::cast_from(0.34146007962811379518e0_f64) * t2268 * t11264 * t6949;
    let t44574 = F::cast_from(0.17073003981405689759e0_f64) * t6305 * t13277;
    let t44576 = F::cast_from(0.34146007962811379518e0_f64) * t6305 * t13268;
    let t44578 = F::cast_from(0.26558006193297739625e0_f64) * t6313 * t13307;
    let t44579 = F::cast_from(0.94850022118920498664e-2_f64) * t42846;
    let t44580 = F::cast_from(0.94850022118920498664e-2_f64) * t42849;
    (t44564, t44572, t44574, t44576, t44578, t44579, t44580)
}
