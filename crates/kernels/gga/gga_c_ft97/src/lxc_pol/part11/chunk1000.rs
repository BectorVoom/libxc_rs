//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1000/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1000<F: Float>(t10577: F, t1775: F, t2778: F, t8282: F, t2767: F, t295: F, t41751: F, t10581: F, t10597: F, t3139: F, t849: F, t2775: F, t10560: F, t10261: F, t10388: F, t10575: F, t10580: F, t2: F, t2681: F, t2682: F, t2739: F, t42071: F, t42075: F, t42088: F, t42096: F, t462: F, t848: F) -> (F,) {
    let t43843 = t1775 * t10577;
    let t43848 = t8282 * t2778;
    let t43850 = t8282 * t2767;
    let t43852 = t41751 * t295;
    let t43860 = t1775 * t10581;
    let t43867 = t1775 * t10597;
    let t43872 = t3139 * t849;
    let t43874 = t8282 * t2775;
    let t43879 = t1775 * t10560;
    let t43881 = -8.0 * t43843 + 40.0 / 9.0 * t462 * t10580 * t42088 - 8.0 / 9.0 * t43848 - 16.0 / 27.0 * t43850 - 80.0 / 81.0 * t462 * t43852 * t42096 + 8.0 * t462 * t2681 * t10575 * t10388 + 40.0 / 81.0 * t43860 - 36.0 * t462 * t10261 * t2 * t2682 * t2739 + 8.0 / 3.0 * t43867 + 8.0 * t462 * t848 * t42071 + 112.0 / 81.0 * t43872 + 16.0 / 9.0 * t43874 + 2.0 * t462 * t848 * t42075 - 16.0 / 9.0 * t43879;
    (t43881,)
}
