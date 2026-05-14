//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 645/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk645<F: Float>(t13157: F, t1457: F, t6060: F, t1445: F, t2087: F, t13124: F, t13127: F, t13132: F, t13134: F, t13138: F, t13140: F, t13144: F, t13147: F, t13152: F, t13156: F, t13048: F, t13075: F, t13123: F) -> (F, F, F) {
    let t13158 = t1457 * t13157;
    let t13160 = 0.21450293971110256001e1 * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = 0.62115540045351614476e2 * t2087 * t13161;
    let t13164 = t13124 - 0.92023022289409799224e1 * t13127 - t13132 + 0.23005755572352449806e2 * t13134 + t13138 + t13140 + t13144 - 0.29792074959875355558e-1 * t13147 - t13152 + t13156 - t13160 - t13163;
    let t13166 = t13048 + t13075 + t13123 + t13164;
    (t13158, t13161, t13166)
}
