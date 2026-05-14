//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 971/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk971<F: Float>(t1013: F, t355: F, t53: F, t7205: F, t136637: F, t6604: F, t6608: F, t2035: F, t32791: F, t105135: F, t137007: F, t138968: F, t138969: F, t138991: F, t138996: F, t139082: F, t139087: F, t139098: F, t139101: F, t139109: F, t139116: F, t139124: F, t147432: F, t147435: F, t147440: F, t147445: F, t2034: F, t23869: F, t3356: F, t3384: F, t3394: F, t34864: F, t8833: F, t8852: F) -> (F, F) {
    let t147453 = t7205 * t355 * t1013 * t53;
    let t147462 = t136637 * t6604;
    let t147465 = t136637 * t6608;
    let t147474 = t2035 * t32791 * t1013;
    let t147492 = 0.58778941170896004276e-1 * t139109 * t147445 + 0.88168411756344006411e-1 * t139124 * t147453 - 0.88168411756344006411e-1 * t139116 * t147453 + 0.41054213886971219988e0 * t105135 * t34864 - 0.45306850413028723348e0 * t8833 * t147435 - 0.84754336316176678532e-1 * t139098 * t147462 + 0.84754336316176678532e-1 * t139101 * t147465 - 0.20527106943485609994e0 * t138968 * t138969 * t3356 + 0.45306850413028723348e0 * t23869 * t147432 + 0.27369475924647479993e1 * t8852 * t147474 + 0.42377168158088339266e-1 * t139082 * t147462 - 0.42377168158088339266e-1 * t139087 * t147465 + 0.45306850413028723348e0 * t23869 * t147440 - 0.20527106943485609994e0 * t138968 * t138969 * t3384 + 0.20527106943485609994e0 * t2034 * t137007 * t138969 * t3394 + 0.70628613596813898777e-2 * t138991 + 0.19592980390298668092e-1 * t138996;
    (t147474, t147492)
}
