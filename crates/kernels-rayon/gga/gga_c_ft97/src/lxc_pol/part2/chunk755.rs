//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 755/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk755(t3219: f64, t379: f64, t11854: f64, t1876: f64, t925: f64, t8557: f64, t110: f64, t8216: f64, t11064: f64, t3214: f64, t3271: f64, t11826: f64, t11829: f64, t11833: f64, t11839: f64, t11843: f64, t11846: f64, t11849: f64, t11851: f64, t1901: f64, t3281: f64, t446: f64, t8393: f64, t8409: f64) -> f64 {
    let t11855 = t3219 * t379;
    let t11856 = t11854 * t11855;
    let t11859 = t925 * t1876;
    let t11860 = t8557 * t11859;
    let t11863 = t8216 * t110;
    let t11864 = t11863 * t11064;
    let t11867 = t3214 * t379;
    let t11868 = t8557 * t11867;
    let t11871 = t3271 * t379;
    let t11872 = t8557 * t11871;
    let t11876 = -t11826 - 2.0_f64 / 9.0_f64 * t1901 * t11829 - 2.0_f64 / 9.0_f64 * t1901 * t11833 - 2.0_f64 / 27.0_f64 * t8393 - 2.0_f64 / 3.0_f64 * t446 * t11839 - 2.0_f64 / 9.0_f64 * t3281 * t11843 - 4.0_f64 / 27.0_f64 * t11846 + t11849 - 2.0_f64 / 9.0_f64 * t446 * t11851 - 4.0_f64 / 9.0_f64 * t1901 * t11856 - 2.0_f64 / 9.0_f64 * t1901 * t11860 - 4.0_f64 / 9.0_f64 * t1901 * t11864 - 2.0_f64 / 9.0_f64 * t1901 * t11868 - 2.0_f64 / 9.0_f64 * t1901 * t11872 - 2.0_f64 / 9.0_f64 * t8409;
    t11876
}
