//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 996/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk996<F: Float>(t10525: F, t10526: F, t47803: F, t6717: F, t6914: F, t12079: F, t2389: F, t12092: F, t2482: F, t9267: F, t40009: F, t41697: F, t41699: F, t41700: F, t41703: F, t41706: F, t41712: F, t41713: F) -> (F,) {
    let t47860 = t10525 * t10526 * t47803;
    let t47864 = t6914 * t6717 * t47803;
    let t47866 = t12079 * t2389;
    let t47869 = t9267 * t12092 * t2482;
    let t47871 = 0.63904876589867916128e-1 * t40009;
    let t47872 = -t41697 + t41699 - 0.21450293971110256001e1 * t47860 - t41700 - 0.46011511144704899612e1 * t41703 - 0.62115540045351614476e2 * t47864 - 0.29792074959875355558e-1 * t47866 - t41706 - t41712 + 0.9585731488480187419e0 * t47869 + t41713 - t47871;
    (t47872,)
}
