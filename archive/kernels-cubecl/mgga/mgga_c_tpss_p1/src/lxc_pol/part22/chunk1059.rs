//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1059/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1059<F: Float>(t11031: F, t3923: F, t11035: F, t242: F, t2751: F, t3758: F, t967: F, t10984: F, t970: F, t11524: F, t11528: F, t11529: F, t11532: F, t11536: F, t2685: F, t2748: F, t3920: F, t3983: F, t925: F) -> F {
    let t11539 = t3923 * t11031;
    let t11542 = t3923 * t11035;
    let t11548 = t242 * t2751 * t3758;
    let t11550 = t967 * t11548 / F::cast_from(3456.0_f64);
    let t11552 = t242 * t970 * t10984;
    let t11555 = -F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t2685 * t3920 - t11524 + t11528 + t925 * t11529 / F::cast_from(108.0_f64) + t925 * t11532 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t925 * t11536 - t925 * t11539 / F::cast_from(72.0_f64) - t925 * t11542 / F::cast_from(144.0_f64) - t2748 * t3983 / F::cast_from(432.0_f64) + t11550 + t967 * t11552 / F::cast_from(4608.0_f64);
    t11555
}
