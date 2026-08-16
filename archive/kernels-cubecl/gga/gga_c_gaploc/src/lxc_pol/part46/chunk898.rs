//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 898/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk898<F: Float>(t10272: F, t2317: F, t6525: F, t12830: F, t1358: F, t31748: F, t4261: F, t9074: F, t3129: F, t31903: F, t10166: F, t9086: F) -> (F, F, F, F, F) {
    let t42579 = t6525 * t10272 * t2317;
    let t42580 = F::cast_from(0.23712505529730124666e-2_f64) * t42579;
    let t42581 = t1358 * t12830;
    let t42582 = F::cast_from(0.94850022118920498664e-2_f64) * t42581;
    let t42584 = t9074 * t4261 * t31748;
    let t42587 = t9074 * t31903 * t3129;
    let t42588 = F::cast_from(0.71137516589190373998e-2_f64) * t42587;
    let t42590 = t9074 * t10166 * t9086;
    (t42580, t42582, t42584, t42588, t42590)
}
