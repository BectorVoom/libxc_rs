//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1233/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1233<F: Float>(t1137: F, t154: F, t17864: F, t18331: F, t20743: F, t2104: F, t2105: F, t2106: F, t21435: F, t21494: F, t21496: F, t21500: F, t21518: F, t21527: F, t276: F, t287: F, t2899: F, t2900: F, t2901: F, t2912: F, t302: F, t5537: F, t5984: F, t735: F, t742: F, t7632: F, t7720: F, t7742: F, t7743: F, t7857: F) -> F {
    let t21533 = -F::cast_from(0.43445671692977333464e-1_f64) * t17864 * t2912 + F::cast_from(0.13719685797782315831e-1_f64) * t5984 * t7720 + F::cast_from(0.45732285992607719436e-2_f64) * t21494 + F::cast_from(0.91464571985215438873e-2_f64) * t21496 - t21500 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t2105 * t1137 * t287 * t5537 - F::cast_from(0.38586616306262763275e-2_f64) * t7742 * t302 * t21435 * t7743 + F::cast_from(0.42874018118069736972e-3_f64) * t2899 * t302 * t2900 * t18331 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t2105 * t7857 * t2106 + F::cast_from(0.12862205435420921092e-2_f64) * t2899 * t302 * t21518 * t2901 + t735 * t7632 / F::cast_from(12.0_f64) - t21527 / F::cast_from(96.0_f64) - t276 * t154 * t742 * t20743 / F::cast_from(96.0_f64);
    t21533
}
