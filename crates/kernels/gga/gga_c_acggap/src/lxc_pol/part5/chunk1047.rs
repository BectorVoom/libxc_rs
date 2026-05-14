//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1047/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1047<F: Float>(t1745: F, t981: F, t14223: F, t5940: F, t1851: F, t3228: F, t1008: F, t5546: F, t5551: F, t1089: F, t1173: F, t12899: F, t1459: F, t16008: F, t16013: F, t16017: F, t16023: F, t16025: F, t1782: F, t418: F, t4680: F, t5617: F, t839: F) -> (F,) {
    let t21012 = t981 * t1745;
    let t21014 = t14223 * t5940;
    let t21016 = t3228 * t1851;
    let t21018 = t1008 * t5546;
    let t21020 = t1008 * t5551;
    let t21030 = -0.21437009059034868486e-3 * t12899 - 0.51448821741683684368e-2 * t418 * t1089 * t1459 * t1782 * t839 + 0.42874018118069736972e-3 * t21012 + 0.16006300097412701803e-1 * t21014 + 0.34299214494455789578e-2 * t21016 + 0.68598428988911579156e-2 * t21018 + 0.68598428988911579156e-2 * t21020 - 0.34299214494455789577e-2 * t16008 - 0.17149607247227894789e-2 * t16013 - 0.17149607247227894789e-2 * t16017 - 0.85748036236139473944e-3 * t16023 + 0.34299214494455789578e-2 * t16025 + 0.68598428988911579156e-2 * t1173 * t4680 * t5617;
    (t21030,)
}
