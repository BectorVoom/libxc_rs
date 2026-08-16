//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1592;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta318<F: Float>(t2778: F, t9303: F, t871: F, t9292: F, t2760: F, t72: F, t686: F, t874: F, t10861: F, t10872: F, t10921: F, t10923: F, t10925: F, t10930: F, t10932: F, t10935: F, t10939: F, t10943: F, t10948: F, t10952: F, t10961: F, t10964: F, t10966: F, t2754: F, t2784: F, t2811: F, t2815: F, t4504: F, t4514: F, t820: F, t837: F, t10918: F, t868: F, t251: F, t9646: F, t22: F, t780: F, t2455: F, t9285: F, t2454: F, t2829: F, t779: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10969, t10971, t10972, t10974, t10976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1592::<F>(t2778, t9303, t871, t9292, t2760, t72, t686, t874, t10861, t10872, t10921, t10923, t10925, t10930, t10932, t10935, t10939, t10943, t10948, t10952, t10961, t10964, t10966, t2754, t2784, t2811, t2815, t4504, t4514, t820, t837);
        let (t10977, t10978, t10981, t10982, t10984, t10985, t10987, t10988) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1593::<F>(t10918, t10976, t868, t251, t9646, t22, t780, t2455, t9285, t2454, t2829, t779);
    (t10969, t10971, t10972, t10974, t10977, t10978, t10981, t10982, t10984, t10985, t10987, t10988)
}
