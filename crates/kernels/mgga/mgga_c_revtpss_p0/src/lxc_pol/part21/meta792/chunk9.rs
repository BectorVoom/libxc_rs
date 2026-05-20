//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2866/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2866<F: Float>(t15199: F, t698: F, t141: F, t51969: F, t930: F, t51973: F, t41329: F, t41361: F, t41363: F, t41369: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F) -> (F, F, F) {
    let t52065 = t698 * t15199;
    let t52068 = t141 * t930 * t51969;
    let t52082 = F::new(4.0) / F::new(9.0) * t51973;
    let t52090 = F::new(8.0) * t51849 - F::new(2.0) / F::new(9.0) * t51853 - F::new(80.0) / F::new(81.0) * t51858 + F::new(2.0) * t51863 + F::new(2.0) * t51867 + F::new(2.0) / F::new(3.0) * t51871 - F::new(8.0) * t51875 + t41329 + F::new(4.0) * t51961 - F::new(10.0) / F::new(9.0) * t51965 + t51967 / F::new(3.0) - t51971 / F::new(3.0) - t52082 + F::new(28.0) / F::new(81.0) * t51978 + F::new(28.0) / F::new(27.0) * t41361 + F::new(8.0) / F::new(9.0) * t41363 - F::new(4.0) / F::new(9.0) * t41369 + F::new(4.0) * t52028 + F::new(40.0) / F::new(9.0) * t52031 + F::new(2.0) * t52033;
    (t52065, t52068, t52090)
}
