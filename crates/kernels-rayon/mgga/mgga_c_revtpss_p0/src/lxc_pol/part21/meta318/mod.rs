//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1592;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta318(t2778: f64, t9303: f64, t871: f64, t9292: f64, t2760: f64, t72: f64, t686: f64, t874: f64, t10861: f64, t10872: f64, t10921: f64, t10923: f64, t10925: f64, t10930: f64, t10932: f64, t10935: f64, t10939: f64, t10943: f64, t10948: f64, t10952: f64, t10961: f64, t10964: f64, t10966: f64, t2754: f64, t2784: f64, t2811: f64, t2815: f64, t4504: f64, t4514: f64, t820: f64, t837: f64, t10918: f64, t868: f64, t251: f64, t9646: f64, t22: f64, t780: f64, t2455: f64, t9285: f64, t2454: f64, t2829: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10969, t10971, t10972, t10974, t10976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1592(t2778, t9303, t871, t9292, t2760, t72, t686, t874, t10861, t10872, t10921, t10923, t10925, t10930, t10932, t10935, t10939, t10943, t10948, t10952, t10961, t10964, t10966, t2754, t2784, t2811, t2815, t4504, t4514, t820, t837);
        let (t10977, t10978, t10981, t10982, t10984, t10985, t10987, t10988) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1593(t10918, t10976, t868, t251, t9646, t22, t780, t2455, t9285, t2454, t2829, t779);
    (t10969, t10971, t10972, t10974, t10977, t10978, t10981, t10982, t10984, t10985, t10987, t10988)
}
