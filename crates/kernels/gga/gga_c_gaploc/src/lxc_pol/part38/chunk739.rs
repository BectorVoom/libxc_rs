//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 739/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk739<F: Float>(t13277: F, t6305: F, t13268: F, t13307: F, t6313: F, t42846: F, t42849: F, t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F) -> (F, F, F, F, F, F) {
    let t44574 = 0.17073003981405689759e0 * t6305 * t13277;
    let t44576 = 0.34146007962811379518e0 * t6305 * t13268;
    let t44578 = 0.26558006193297739625e0 * t6313 * t13307;
    let t44579 = 0.94850022118920498664e-2 * t42846;
    let t44580 = 0.94850022118920498664e-2 * t42849;
    let t44590 = (21.0 / 256.0 * t39624 + 357.0 / 8192.0 * t39626 - 189.0 / 131072.0 * t39632 + 189.0 / 8388608.0 * t39637 - 63.0 / 8388608.0 * t39642 + 63.0 / 131072.0 * t39646 - 119.0 / 8192.0 * t39648 - 7.0 / 256.0 * t39650) * t471;
    (t44574, t44576, t44578, t44579, t44580, t44590)
}
