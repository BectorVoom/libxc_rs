//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta843 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3042;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3043;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3044;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3045;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3046;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3047;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta843<F: Float>(t18205: F, t2250: F, t11145: F, t123: F, t43780: F, t43782: F, t43816: F, t44320: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t2244: F, t43791: F, t5392: F, t18216: F, t690: F, t18212: F, t18210: F, t3240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t63372, t63374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3042::<F>(t18205, t2250, t11145, t123);
        let t63376 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3043::<F>(t43780, t43782, t43816, t44320, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let (t63378, t63380) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3044::<F>(t2244, t43791, t5392, t11145, t123);
        let t63382 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3045::<F>(t18216, t690);
        let t63384 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3046::<F>(t18212, t690);
        let (t63386, t63388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3047::<F>(t18210, t2250, t123, t3240);
        let (t63390, t63392) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3048::<F>(t18205, t2244, t123, t3240);
    (t63372, t63374, t63376, t63378, t63380, t63382, t63384, t63386, t63388, t63390, t63392)
}
