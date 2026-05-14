//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 845/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk845<F: Float>(t1022: F, t12250: F, t1457: F, t2009: F, t2021: F, t2103: F, t43994: F, t44001: F, t44004: F, t44009: F, t45803: F, t45806: F, t45809: F, t45812: F, t45817: F, t45820: F, t45823: F, t45826: F, t45831: F, t45837: F, t45848: F, t45856: F, t45863: F, t47450: F, t50092: F) -> (F,) {
    let t50272 = t12250 * t1022;
    let t50276 = t45803 + t45806 + t45809 + t45812 - t45817 - t45820 + t45823 - 0.9585731488480187419e0 * t45826 - t45831 + t45837 - 0.11916829983950142223e0 * t47450 + t45848 + 0.14300195980740170668e1 * t2103 * t1457 * t50092 - t45856 - t43994 + 0.63904876589867916127e-1 * t44001 + 0.38342925953920749676e1 * t44004 + 0.63904876589867916127e-1 * t44009 - 0.71500979903700853338e0 * t2021 * t50272 * t2009 - t45863;
    (t50276,)
}
