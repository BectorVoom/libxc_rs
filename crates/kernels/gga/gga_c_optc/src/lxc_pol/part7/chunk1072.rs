//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1072/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1072<F: Float>(t2414: F, t216: F, t2374: F, t2417: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F, t23682: F) -> (F, F, F, F) {
    let t24300 = t2414 * t2414;
    let t24302 = t216 / t24300;
    let t24303 = t2374 * t2374;
    let t24304 = t2417 * t2417;
    let t24305 = 1.0 / t24304;
    let t24308 = 0.24954977986735470917e5 * t24302 * t24303 * t24305;
    let t24320 = -0.22249999999999999999e0 * t23605 + 0.22249999999999999999e0 * t23670 - 0.18541666666666666666e-1 * t23608 - 0.24722222222222222222e-1 * t23673 - 0.61805555555555555555e-1 * t23676 + 0.2225e0 * t23612 - 0.33375e0 * t23679 + 0.49444444444444444445e-1 * t23614 + 0.74166666666666666668e-1 * t23616 - 0.74166666666666666668e-1 * t23653 + 0.24722222222222222222e-1 * t23655;
    let t24321 = 0.96141975308641975307e-1 * t23682;
    (t24303, t24308, t24320, t24321)
}
