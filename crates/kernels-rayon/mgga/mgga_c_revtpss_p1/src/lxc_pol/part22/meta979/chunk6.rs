//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3296/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3296(t10811: f64, t18334: f64, t18629: f64, t10777: f64, t10779: f64, t14671: f64, t18637: f64, t50412: f64, t6035: f64, t14586: f64, t14767: f64, t14785: f64, t14791: f64, t14894: f64, t1559: f64, t18493: f64, t18498: f64, t2745: f64, t36833: f64, t4362: f64, t50418: f64, t50423: f64, t50474: f64, t50560: f64, t51014: f64, t51049: f64, t51178: f64, t837: f64) -> f64 {
    let t62475 = t10811 * t18334;
    let t62494 = t10811 * t18629;
    let t62498 = t10777 * t10779 * t14671 * t18637;
    let t62502 = t10777 * t10779 * t50412 * t6035;
    let t62504 = -0.34299214494455789578e-2_f64 * t4362 * t14791 * t14586 * t50418 + 0.51448821741683684366e-1_f64 * t2745 * t51014 * t18493 * t837 - 0.17149607247227894789e-1_f64 * t2745 * t14785 * t18498 * t837 - 0.16006300097412701803e-1_f64 * t62475 - 0.85748036236139473945e-2_f64 * t2745 * t14785 * t1559 * t14767 - 0.51448821741683684367e-2_f64 * t14894 * t36833 * t50474 * t51049 + 0.17149607247227894789e-2_f64 * t2745 * t14791 * t50560 * t6035 - 0.10289764348336736873e-1_f64 * t4362 * t14791 * t14586 * t50423 + 0.57165357490759649296e-3_f64 * t51178 - 0.80031500487063509014e-2_f64 * t62494 + 0.2032800112371413129e-3_f64 * t62498 + 0.2032800112371413129e-3_f64 * t62502;
    t62504
}
