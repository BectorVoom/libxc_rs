//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 593/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk593<F: Float>(t2722: F, t5025: F, t4937: F, t914: F, t4929: F, t4933: F, t2813: F, t2721: F, t2729: F, t2745: F, t2751: F, t2758: F, t2773: F, t2778: F, t2812: F, t3892: F, t3897: F, t3947: F, t3952: F, t4972: F, t4976: F, t4980: F, t4998: F, t5003: F, t5008: F, t5012: F, t5017: F, t5022: F, t913: F, t930: F, t940: F, t953: F) -> (F,) {
    let t5026 = t2722 * t5025;
    let t5037 = t914 * t4937;
    let t5040 = t914 * t4929;
    let t5043 = t914 * t4933;
    let t5046 = t2813 * t5025;
    let t5049 = 0.779739765264702906e1 * t3947 + 0.75734008510040627574e0 * t3952 + 0.23229342182245570105e2 * t2751 * t4998 - 0.77431140607485233683e1 * t2758 * t5003 + 0.5848048239485271795e1 * t940 * t5008 + 0.8790987341241436962e3 * t2773 * t5012 - 0.4395493670620718481e3 * t2778 * t5017 + 0.11360101276506094136e1 * t913 * t5022 - t2729 - t2745 + 0.75734008510040627574e0 * t2721 * t5026 + 0.6717427261115226305e-2 * t3892 + 0.19318136643975017455e-1 * t3897 - 0.10076140891672839458e-1 * t953 * t4976 + 0.50380704458364197288e-2 * t953 * t4980 + 0.83967840763940328814e-2 * t953 * t4972 + 0.28977204965962526182e-1 * t930 * t5037 + 0.38636273287950034909e-1 * t930 * t5040 - 0.57954409931925052364e-1 * t930 * t5043 + 0.779739765264702906e1 * t2812 * t5046;
    (t5049,)
}
