//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta802(t12940: f64, t58994: f64, t12606: f64, t4194: f64, t4195: f64, t12908: f64, t16713: f64, t12939: f64, t5392: f64, t607: f64, t750: f64, t157: f64, t4196: f64, t46447: f64, t41274: f64, t39658: f64, t41254: f64, t41258: f64, t41262: f64, t58983: f64, t58985: f64, t58986: f64, t58987: f64, t58988: f64, t58989: f64, t58990: f64, t58991: f64, t58993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t58996, t58999, t59001, t59005, t59008) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789(t12940, t58994, t12606, t4194, t4195, t12908, t16713, t12939, t5392, t607, t750, t157, t4196, t46447);
        let (t59009, t59010) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790(t41274, t39658, t41254, t41258, t41262, t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58996, t58999, t59001, t59005, t59008);
    (t58996, t58999, t59001, t59005, t59008, t59009, t59010)
}
