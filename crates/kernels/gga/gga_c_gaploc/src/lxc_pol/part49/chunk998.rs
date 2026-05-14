//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 998/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk998<F: Float>(t13791: F, t1429: F, t549: F, t41734: F, t41735: F, t41736: F, t41737: F, t41741: F, t41743: F, t41744: F, t41747: F, t41752: F, t41753: F, t41754: F, t40116: F, t1445: F, t1450: F, t447: F, t46919: F) -> (F, F, F) {
    let t47892 = t1429 * t549 * t13791;
    let t47894 = -t41734 - t41735 - t41736 + t41737 + 0.29792074959875355558e-1 * t47892 + t41741 + t41743 - t41744 + t41747 - t41752 - t41753 + t41754;
    let t47895 = 0.85206502119823888171e-1 * t40116;
    let t47900 = 0.23005755572352449806e1 * t1450 * t1445 * t46919 * t447;
    (t47894, t47895, t47900)
}
