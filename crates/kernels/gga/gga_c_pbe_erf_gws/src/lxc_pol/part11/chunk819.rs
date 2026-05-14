//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 819/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk819<F: Float>(t1903: F, t1924: F, t1917: F, t703: F, t712: F, t5: F, t922: F, t168: F, t270: F, t153: F, t18046: F, t274: F, t1383: F, t762: F, t4598: F, t528: F) -> (F, F, F, F, F, F, F) {
    let t18315 = 4.0 / 9.0 * t1924 * t1903;
    let t18318 = 0.5402469135802469136e-1 * t712 * t703 * t1917;
    let t18344 = t5 * t922;
    let t18347 = 0.90790602394455990432e0 * t168 * t18344 * t270;
    let t18359 = 0.19192636997366703204e2 * t153 * t18046 * t274;
    let t18363 = 0.10051538464260528225e1 * t762 * t1383;
    let t18372 = 0.33505128214201760751e0 * t528 * t4598;
    (t18315, t18318, t18344, t18347, t18359, t18363, t18372)
}
