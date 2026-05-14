//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 796/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk796<F: Float>(t12974: F, t12916: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12985: F, t12989: F, t12993: F, t13010: F, t1175: F, t12970: F, t12992: F, t13244: F, t13247: F, t13250: F, t13253: F, t1355: F, t306: F, t3559: F, t3587: F, t3599: F, t3602: F) -> (F,) {
    let t13263 = 0.12841111111111111111e-1 * t12974;
    let t13274 = 0.14865e-1 * t13010 - 0.2973e-1 * t12916 + 0.1982e-1 * t12993 - t13263 - 0.55033333333333333332e-2 * t12929 + 0.27516666666666666666e-2 * t12933 - 0.82549999999999999999e-2 * t12948 + 0.41274999999999999999e-2 * t12931 - 0.45861111111111111112e-2 * t12922 + 0.1651e-1 * t12954 - 0.82550000000000000001e-2 * t12985 - 0.24765e-1 * t12959 + 0.24765e-1 * t12989 - 0.41275e-2 * t12927;
    let t13277 = 3.0 / 16.0 * t13244 * t12970 - 3.0 / 8.0 * t13247 * t3559 - 3.0 / 8.0 * t3599 * t13250 + 3.0 / 4.0 * t13253 * t1175 + 3.0 / 4.0 * t3602 * t3587 + t1355 * t12992 / 4.0 + t306 * t13274 / 2.0;
    (t13277,)
}
