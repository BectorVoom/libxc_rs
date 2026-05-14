//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1067/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1067<F: Float>(t26334: F, t686: F, t72: F, t7289: F, t7284: F, t96282: F, t2027: F, t2028: F, t2103: F, t25909: F, t26282: F, t4078: F, t545: F, t7532: F, t94643: F, t96277: F, t96280: F, t96284: F, t96287: F, t96289: F, t96292: F, t96294: F, t96296: F, t96298: F, t96362: F) -> (F, F) {
    let t96370 = t26334 * t72 * t686;
    let t96371 = t7289 * t96370;
    let t96374 = 0.22487184191643109717e-1 * t7284 * t96282;
    let t96377 = -0.28912093960683998208e-1 * t96277 - 0.10281140612419229763e-1 * t96280 - t96284 - 0.13010442282307799193e1 * t25909 * t7532 - 0.68549505033305214441e-2 * t96287 + 0.51405703062096148812e-1 * t96289 + 0.43368140941025997312e-1 * t96292 - 0.77108554593144223218e-1 * t96294 - 0.86736281882051994623e-1 * t96296 + 0.28912093960683998208e-1 * t96298 - 0.4336814094102599731e0 * t2027 * t2028 * t545 * t96362 + 0.39512695097613069591e1 * t26282 * t4078 - 0.38554277296572111609e-1 * t96371 + t96374 - 0.4336814094102599731e0 * t94643 * t2103;
    (t96370, t96377)
}
