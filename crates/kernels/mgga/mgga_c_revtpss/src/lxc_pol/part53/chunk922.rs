//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 922/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk922<F: Float>(t27384: F, t27799: F, t1113: F, t1583: F, t33: F, t4537: F, t1711: F, t775: F, t890: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t27158: F, t27364: F, t27368: F, t27382: F, t27407: F, t27764: F, t27770: F, t27773: F, t27777: F, t27793: F, t7087: F, t7091: F, t7200: F, t7207: F, t7783: F, t7862: F, t7869: F) -> (F, F, F, F, F, F) {
    let t27800 = t27799 * t27384;
    let t27802 = t1113 * t1583;
    let t27806 = t33 * t4537;
    let t27810 = t1711 * t775;
    let t27817 = t1711 * t890;
    let t27821 = F::new(3.0) * t27158 * t27764 + F::new(3.0) / F::new(2.0) * t2403 * t7087 * t7862 - F::new(3.0) / F::new(2.0) * t25206 * t27770 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t27773 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t27777 + F::new(3.0) / F::new(2.0) * t2403 * t7783 * t7200 + t1940 * t27364 * t33 / F::new(2.0) - t1940 * t27368 * t7207 / F::new(2.0) + t1940 * t7783 * t1113 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t25206 * t27793 - t1940 * t25440 * t7869 / F::new(2.0) + t27382 * t27800 - t1940 * t7091 * t27802 / F::new(2.0) - t1940 * t7091 * t27806 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t27810 + t1940 * t7087 * t1711 / F::new(2.0) - t1940 * t7091 * t27817 / F::new(2.0) - t27407;
    (t27800, t27802, t27806, t27810, t27817, t27821)
}
