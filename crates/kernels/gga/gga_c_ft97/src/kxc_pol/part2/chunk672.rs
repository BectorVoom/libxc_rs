//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 672/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk672<F: Float>(t8232: F, t955: F, t1882: F, t3227: F, t3291: F, t379: F, t447: F, t1852: F, t463: F, t3219: F, t1876: F, t925: F, t8557: F, t110: F, t8216: F, t11064: F) -> (F, F, F, F, F, F) {
    let t11846 = t8232 * t955;
    let t11849 = 2.0 / 9.0 * t1882 * t3227;
    let t11851 = t447 * t3291 * t379;
    let t11854 = t463 * t1852;
    let t11855 = t3219 * t379;
    let t11856 = t11854 * t11855;
    let t11859 = t925 * t1876;
    let t11860 = t8557 * t11859;
    let t11863 = t8216 * t110;
    let t11864 = t11863 * t11064;
    (t11846, t11849, t11851, t11856, t11860, t11864)
}
