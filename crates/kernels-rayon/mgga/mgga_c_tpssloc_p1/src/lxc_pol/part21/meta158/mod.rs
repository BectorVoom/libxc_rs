//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1027;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1028;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1029;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1030;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1031;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta158(t2600: f64, t541: f64, t1329: f64, t3726: f64, t119: f64, t3734: f64, t210: f64, t3719: f64, t225: f64, t3752: f64, t554: f64, t1332: f64, t68: f64, t1340: f64, t1333: f64, t1358: f64, t1362: f64, t1337: f64, t551: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3762, t3763, t3766, t3770, t3773) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1027(t2600, t541, t1329, t3726, t119, t3734, t210, t3719, t225, t3752);
        let (t3774, t3777) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1028(t3773, t554, t1332, t68);
        let t3778 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1029(t1340, t3777);
        let (t3781, t3783) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1030(t1333, t1358, t1362, t3777);
        let t3787 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1031(t1337, t551);
        let t3788 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1032(t236, t3787);
    (t3762, t3763, t3766, t3770, t3773, t3774, t3777, t3778, t3781, t3783, t3787, t3788)
}
