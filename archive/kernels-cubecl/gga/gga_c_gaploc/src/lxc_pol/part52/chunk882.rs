//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 882/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk882<F: Float>(t2684: F, t45320: F, t7585: F, t45369: F, t7427: F, t7573: F, t10930: F, t10931: F, t326: F, t45423: F, t825: F, t11832: F, t1445: F, t2530: F, t5748: F) -> (F, F, F, F, F) {
    let t45766 = F::cast_from(0.87421871174939309262e2_f64) * t2684 * t7585 * t45320;
    let t45772 = F::cast_from(0.37959496694381542179e3_f64) * t7427 * t7573 * t45369;
    let t45775 = F::cast_from(0.38649669361552115674e3_f64) * t10930 * t10931 * t45369;
    let t45778 = F::cast_from(0.92023022289409799224e1_f64) * t825 * t326 * t45423;
    let t45785 = F::cast_from(0.27606906686822939767e2_f64) * t5748 * t1445 * t11832 * t2530;
    (t45766, t45772, t45775, t45778, t45785)
}
