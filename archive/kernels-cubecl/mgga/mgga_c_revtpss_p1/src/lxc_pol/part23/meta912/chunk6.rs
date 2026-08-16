//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2938/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2938<F: Float>(t52035: F, t52037: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t77539: F, t77543: F, t77547: F, t77799: F, t52128: F, t52751: F, t63447: F, t63453: F, t63459: F, t77802: F, t77804: F, t77806: F, t77810: F, t77813: F, t77816: F, t77819: F) -> (F, F) {
    let t78049 = -F::cast_from(0.53814e1_f64) * t77539 + F::cast_from(0.17938e1_f64) * t77543 + F::cast_from(0.17938e1_f64) * t77547 - F::cast_from(0.11958666666666666667e1_f64) * t63338 + F::cast_from(0.39862222222222222222e0_f64) * t63340 + F::cast_from(0.33218518518518518518e0_f64) * t63342 + F::cast_from(0.17938e1_f64) * t63361 - F::cast_from(0.11958666666666666667e1_f64) * t63371 + F::cast_from(0.79724444444444444446e0_f64) * t52035 - F::cast_from(0.26574814814814814815e0_f64) * t52037 + F::cast_from(0.3071625e0_f64) * t77799;
    let t78061 = F::cast_from(0.1898925e1_f64) * t77802 - F::cast_from(0.32862666666666666666e0_f64) * t77804 + F::cast_from(0.54771111111111111112e-1_f64) * t77806 - t52751 + F::cast_from(0.73028148148148148149e0_f64) * t52128 + F::cast_from(0.197176e1_f64) * t77810 - F::cast_from(0.147882e1_f64) * t77813 + F::cast_from(0.49293999999999999999e0_f64) * t77816 + F::cast_from(0.49293999999999999999e0_f64) * t77819 + F::cast_from(0.29896666666666666667e0_f64) * t63447 - F::cast_from(0.26574814814814814815e0_f64) * t63453 + F::cast_from(0.79724444444444444444e0_f64) * t63459;
    (t78049, t78061)
}
