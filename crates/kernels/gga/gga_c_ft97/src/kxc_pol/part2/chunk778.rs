//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 778/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk778<F: Float>(t13780: F, t13390: F, t2354: F, t446: F, t13292: F, t3281: F, t13296: F, t724: F, t13301: F, t1882: F, t3692: F, t13764: F, t13768: F, t13772: F, t13775: F, t13778: F) -> (F, F, F, F, F, F) {
    let t13781 = 2.0 / 27.0 * t13780;
    let t13782 = t2354 * t13390;
    let t13783 = t446 * t13782;
    let t13785 = t2354 * t13292;
    let t13786 = t3281 * t13785;
    let t13788 = t724 * t13296;
    let t13789 = t446 * t13788;
    let t13791 = t724 * t13301;
    let t13792 = t3281 * t13791;
    let t13794 = t1882 * t3692;
    let t13795 = 4.0 / 81.0 * t13794;
    let t13796 = -t13764 / 12.0 + t13768 / 8.0 - t13772 / 6.0 + t13775 / 9.0 + 2.0 / 27.0 * t13778 - t13781 + 2.0 / 9.0 * t13783 - 4.0 / 9.0 * t13786 + 2.0 / 9.0 * t13789 - 8.0 / 9.0 * t13792 + t13795;
    (t13783, t13786, t13789, t13792, t13794, t13796)
}
