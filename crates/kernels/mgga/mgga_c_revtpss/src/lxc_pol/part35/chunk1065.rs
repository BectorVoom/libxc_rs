//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1065/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1065<F: Float>(t102378: F, t102386: F, t108282: F, t109631: F, t109633: F, t109647: F, t109651: F, t114485: F, t114621: F, t115166: F, t2097: F, t2103: F, t22974: F, t25930: F, t26304: F, t27837: F, t28899: F, t30227: F, t30279: F, t6919: F, t7295: F, t8100: F, t94656: F, t94683: F, t96401: F, t9994: F) -> (F,) {
    let t115209 = 0.43368140941025997312e-1 * t109631 - 0.77108554593144223218e-1 * t109633 + 0.10408353825846239354e2 * t7295 * t94656 * t2097 * t22974 - 0.38554277296572111609e-1 * t109647 + 0.58544643236296698113e-1 * t109651 - 0.26020884564615598386e1 * t27837 * t30227 - 0.4336814094102599731e0 * t114485 * t2103 - 0.51405703062096148812e-1 * t102378 + t96401 + 0.68549505033305214441e-2 * t102386 - 0.26020884564615598386e1 * t25930 * t26304 * t114621 - 0.19756347548806534796e1 * t28899 * t6919 + 0.13010442282307799193e1 * t108282 * t8100 - 0.78062653693846795158e1 * t27837 * t30279 + 0.26020884564615598386e1 * t7295 * t94683 * t115166 * t9994;
    (t115209,)
}
