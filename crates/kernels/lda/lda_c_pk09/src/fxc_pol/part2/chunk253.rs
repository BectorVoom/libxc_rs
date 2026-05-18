//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 253/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk253<F: Float>(t1106: F, t721: F, t920: F, t924: F, t612: F, t616: F, t1076: F, t1095: F, t1100: F, t1101: F, t626: F, t636: F, t709: F, t713: F, t894: F, t899: F, t906: F, t910: F, t914: F, t98: F) -> (F, F, F, F, F, F) {
    let t1108 = t1106 * t721 / F::new(6.0);
    let t1114 = F::new(0.01233429741534199) * t920;
    let t1115 = F::new(0.14975624337724558) * t924;
    let t1116 = F::new(0.10237773105191754) * t612;
    let t1117 = F::new(0.06825182070127836) * t616;
    let t1120 = t1076 * t713 / F::new(6.0) + t1076 * t709 / F::new(6.0) - t1095 * t98 / F::new(6.0) - t1100 - t1101 * t713 / F::new(6.0) - t1101 * t709 / F::new(6.0) + t1108 + F::new(0.01233429741534199) * t894 - F::new(0.01233429741534199) * t899 - F::new(0.01233429741534199) * t906 - F::new(0.14975624337724558) * t910 - F::new(0.14975624337724558) * t914 - t1114 - t1115 - t1116 - t1117 - F::new(0.10237773105191754) * t626 - F::new(0.10237773105191754) * t636;
    (t1108, t1114, t1115, t1116, t1117, t1120)
}
