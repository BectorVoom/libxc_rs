//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1236/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1236<F: Float>(t32897: F, t6066: F, t6111: F, t10811: F, t7772: F, t2976: F, t7503: F, t10820: F, t818: F, t825: F, t22706: F, t2684: F) -> (F, F, F, F, F) {
    let t32900 = F::cast_from(0.85801175884441024006e1_f64) * t6111 * t6066 * t32897;
    let t32902 = F::cast_from(0.17875244975925213335e2_f64) * t10811 * t7772;
    let t32903 = t2976 * t7503;
    let t32904 = F::cast_from(0.89376224879626066674e-1_f64) * t32903;
    let t32907 = F::cast_from(0.24539472610509279794e2_f64) * t825 * t818 * t10820;
    let t32910 = F::cast_from(0.11656249489991907902e3_f64) * t2684 * t22706 * t10820;
    (t32900, t32902, t32904, t32907, t32910)
}
