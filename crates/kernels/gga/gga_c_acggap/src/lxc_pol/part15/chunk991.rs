//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 991/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk991<F: Float>(t1983: F, t30127: F, t7586: F, t8791: F, t1181: F, t21143: F, t604: F, t7493: F, t30786: F, t30790: F, t1992: F, t5606: F, t7585: F) -> (F, F, F, F, F) {
    let t34957 = t30127 * t7586 * t1983 * t8791;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34986 = F::new(0.21437009059034868486e-3) * t30786;
    let t34987 = F::new(0.28582678745379824648e-3) * t30790;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    (t34957, t34961, t34986, t34987, t34990)
}
