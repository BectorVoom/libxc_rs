//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 819/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk819(t2: f64, t3408: f64, t1985: f64, t558: f64, t1775: f64, t3520: f64, t11717: f64, t3510: f64, t2075: f64, t3518: f64, t12839: f64, t12840: f64, t12843: f64, t12846: f64, t12850: f64, t12852: f64, t12855: f64, t462: f64, t9178: f64, t9179: f64, t9188: f64, t9190: f64, t9202: f64, t9205: f64, t9207: f64, t9209: f64, t9241: f64) -> f64 {
    let t12858 = t2 * t3408;
    let t12860 = t1985 * t12858 * t558;
    let t12864 = 4.0_f64 / 3.0_f64 * t1775 * t3520;
    let t12865 = t11717 * t3510;
    let t12868 = t1985 * t3518 * t2075;
    let t12878 = t12839 + 2.0_f64 / 3.0_f64 * t462 * t12840 + t462 * t12843 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t462 * t12846 - t12850 - 8.0_f64 / 9.0_f64 * t9179 - 4.0_f64 / 27.0_f64 * t12852 - 6.0_f64 * t462 * t12855 + 4.0_f64 * t462 * t12860 - t12864 + 22.0_f64 / 9.0_f64 * t12865 + 2.0_f64 * t462 * t12868 - t9178 - 2.0_f64 / 3.0_f64 * t9209 + t9241 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t9190 - 8.0_f64 / 27.0_f64 * t9202 + t9205 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t9207 - 2.0_f64 / 9.0_f64 * t9188;
    t12878
}
