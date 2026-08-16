//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2015/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2015(t11984: f64, t1307: f64, t1388: f64, t15868: f64, t15872: f64, t15876: f64, t15878: f64, t15880: f64, t15883: f64, t15887: f64, t15888: f64, t15889: f64, t15891: f64, t15894: f64, t15896: f64, t15898: f64, t15899: f64, t3698: f64, t3914: f64, t5126: f64, t5160: f64, t5161: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64) -> f64 {
    let t15903 = 12.0_f64 * t1307 * t15883 * t5126 - 2.0_f64 * t1388 * t15868 * t5160 + 2.0_f64 * t15899 * t3698 * t5160 - t3914 * t5160 * t5161 + 6.0_f64 * t15872 * t5126 - t11984 + t15876 - t15878 + t15880 - t15887 - t15888 - t15889 - t15891 - t15894 - t15896 - t15898 - t9457 + t9476 + t9484 + t9780;
    t15903
}
