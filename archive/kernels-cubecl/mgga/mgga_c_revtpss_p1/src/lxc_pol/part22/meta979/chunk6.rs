//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3296/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3296<F: Float>(t10811: F, t18334: F, t18629: F, t10777: F, t10779: F, t14671: F, t18637: F, t50412: F, t6035: F, t14586: F, t14767: F, t14785: F, t14791: F, t14894: F, t1559: F, t18493: F, t18498: F, t2745: F, t36833: F, t4362: F, t50418: F, t50423: F, t50474: F, t50560: F, t51014: F, t51049: F, t51178: F, t837: F) -> F {
    let t62475 = t10811 * t18334;
    let t62494 = t10811 * t18629;
    let t62498 = t10777 * t10779 * t14671 * t18637;
    let t62502 = t10777 * t10779 * t50412 * t6035;
    let t62504 = -F::cast_from(0.34299214494455789578e-2_f64) * t4362 * t14791 * t14586 * t50418 + F::cast_from(0.51448821741683684366e-1_f64) * t2745 * t51014 * t18493 * t837 - F::cast_from(0.17149607247227894789e-1_f64) * t2745 * t14785 * t18498 * t837 - F::cast_from(0.16006300097412701803e-1_f64) * t62475 - F::cast_from(0.85748036236139473945e-2_f64) * t2745 * t14785 * t1559 * t14767 - F::cast_from(0.51448821741683684367e-2_f64) * t14894 * t36833 * t50474 * t51049 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14791 * t50560 * t6035 - F::cast_from(0.10289764348336736873e-1_f64) * t4362 * t14791 * t14586 * t50423 + F::cast_from(0.57165357490759649296e-3_f64) * t51178 - F::cast_from(0.80031500487063509014e-2_f64) * t62494 + F::cast_from(0.2032800112371413129e-3_f64) * t62498 + F::cast_from(0.2032800112371413129e-3_f64) * t62502;
    t62504
}
