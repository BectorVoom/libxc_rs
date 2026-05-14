//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 718/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk718<F: Float>(t7335: F, t8349: F, t7508: F, t8533: F, t2134: F, t27: F, t3118: F, t551: F, t270: F, t574: F, t290: F, t2010: F, t7755: F, t1664: F, t7556: F, t2012: F, t7349: F) -> (F, F, F, F, F, F, F, F) {
    let t38757 = t7335 * t8349;
    let t38775 = t7508 * t8533;
    let t38776 = 0.18183107769496894486e-1 * t38775;
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38815 = t574 * t270;
    let t38816 = t290 * t38815;
    let t38818 = t2010 * t7755 * t38816;
    let t38819 = 0.72042316457491791906e-3 * t38818;
    let t38820 = t1664 * t7556;
    let t38822 = t7349 * t2012 * t38820;
    (t38757, t38776, t38784, t38815, t38816, t38819, t38820, t38822)
}
