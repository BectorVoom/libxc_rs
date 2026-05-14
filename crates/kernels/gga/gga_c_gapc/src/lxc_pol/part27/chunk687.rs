//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 687/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk687<F: Float>(t3949: F, t8676: F, t8674: F, t5462: F, t8673: F, t3954: F, t154: F, t125: F, t1736: F, t190: F, t1649: F, t1026: F, t1754: F, t205: F, t1587: F, t1720: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8677 = t8676 * t3949;
    let t8678 = t8674 * t8677;
    let t8680 = t5462 * t8673;
    let t8681 = t8676 * t3954;
    let t8682 = t8680 * t8681;
    let t8684 = t5462 * t154;
    let t8685 = t1736 * t125;
    let t8686 = t8685 * t190;
    let t8687 = t8686 * t1649;
    let t8688 = t8684 * t8687;
    let t8690 = t1754 * t1026;
    let t8691 = t8690 * t205;
    let t8693 = t1720 * t1587;
    (t8677, t8678, t8681, t8682, t8684, t8685, t8686, t8687, t8688, t8691, t8693)
}
