//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1107/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1107<F: Float>(t10915: F, t22242: F, t32514: F, t24321: F, t787: F, t9824: F, t1445: F, t32223: F, t833: F, t32219: F, t2615: F, t326: F, t14667: F, t2365: F, t25289: F, t10938: F, t1980: F, t2028: F) -> (F, F, F, F, F, F, F) {
    let t33021 = 0.42900587942220512002e1 * t22242 * t10915 * t32514;
    let t33023 = t787 * t24321 * t9824;
    let t33024 = 0.14896037479937677779e-1 * t33023;
    let t33030 = 0.11502877786176224903e2 * t833 * t1445 * t32223;
    let t33033 = 0.23005755572352449806e2 * t833 * t1445 * t32219;
    let t33041 = 0.18404604457881959845e2 * t2615 * t326 * t32514;
    let t33047 = t14667 * t2365 * t25289;
    let t33048 = 0.29792074959875355558e-1 * t33047;
    let t33055 = 0.79445533226334281486e-1 * t1980 * t10938 * t2028;
    (t33021, t33024, t33030, t33033, t33041, t33048, t33055)
}
