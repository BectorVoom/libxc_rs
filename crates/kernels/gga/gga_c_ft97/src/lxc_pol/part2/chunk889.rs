//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 889/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk889<F: Float>(t13791: F, t3281: F, t1882: F, t3692: F, t13764: F, t13768: F, t13772: F, t13775: F, t13778: F, t13781: F, t13783: F, t13786: F, t13789: F) -> (F, F, F) {
    let t13792 = t3281 * t13791;
    let t13794 = t1882 * t3692;
    let t13795 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13794;
    let t13796 = -t13764 / F::cast_from(12.0_f64) + t13768 / F::cast_from(8.0_f64) - t13772 / F::cast_from(6.0_f64) + t13775 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13778 - t13781 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13783 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13786 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13789 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13792 + t13795;
    (t13792, t13794, t13796)
}
