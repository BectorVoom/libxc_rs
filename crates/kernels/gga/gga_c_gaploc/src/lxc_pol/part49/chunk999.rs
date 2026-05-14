//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 999/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk999<F: Float>(t13728: F, t4614: F, t597: F, t41759: F, t41761: F, t41767: F, t41769: F, t41773: F, t41777: F, t41781: F, t41783: F, t41787: F, t47895: F, t47900: F, t1445: F, t46915: F, t574: F) -> (F, F) {
    let t47902 = t597 * t4614 * t13728;
    let t47904 = -t47895 - t41759 + t41761 + t41767 - 0.92023022289409799224e1 * t41769 - t41773 + t41777 + t41781 - t41783 - t41787 - t47900 + 0.15337170381568299871e2 * t47902;
    let t47912 = 0.46011511144704899612e1 * t574 * t1445 * t46915;
    (t47904, t47912)
}
