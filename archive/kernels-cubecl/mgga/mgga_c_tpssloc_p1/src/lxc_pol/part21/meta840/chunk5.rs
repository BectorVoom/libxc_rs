//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3018/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018<F: Float>(t18053: F, t225: F, t4693: F, t10160: F, t1049: F, t1052: F, t1055: F, t1066: F, t11010: F, t14529: F, t14552: F, t14555: F, t14659: F, t17588: F, t17875: F, t18047: F, t18166: F, t3020: F, t3026: F, t3174: F, t3207: F, t349: F, t388: F, t4557: F, t4665: F, t4694: F, t5914: F, t5920: F, t61646: F, t62914: F, t62953: F, t62988: F, t63022: F, t63058: F, t63095: F, t63133: F, t63168: F, t63198: F, t990: F) -> F {
    let t63215 = t18053 * t225;
    let t63220 = t4693 * t4693;
    let t63235 = F::cast_from(8.0_f64) * t14529 * t4665 - F::cast_from(2.0_f64) * t61646 * t1066 + t349 * t62914 * t388 - t1052 * t1055 * (t62953 + t62988 + t63022 + t63058 + t63095 + t63133 + t63168 + t63198) + F::cast_from(2.0_f64) * t17875 * t1049 * t388 + t3020 * t5914 * t388 + F::cast_from(8.0_f64) * t14555 * t4665 - F::cast_from(2.0_f64) * t3026 * t18166 - F::cast_from(4.0_f64) * t14552 * t4694 - F::cast_from(2.0_f64) * t63215 * t1066 + F::cast_from(8.0_f64) * t14552 * t4665 + F::cast_from(4.0_f64) * t1052 * t3174 * t63220 - F::cast_from(2.0_f64) * t4557 * t14659 + F::cast_from(2.0_f64) * t11010 * t5920 - F::cast_from(2.0_f64) * t17588 * t3207 + F::cast_from(2.0_f64) * t990 * t18047 * t388 + F::cast_from(4.0_f64) * t10160 * t5920;
    t63235
}
