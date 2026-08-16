//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2782/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2782(t1561: f64, t40360: f64, t14843: f64, t40864: f64, t10779: f64, t14931: f64, t1548: f64, t2724: f64, t10811: f64, t14693: f64, t40850: f64, t40851: f64, t40855: f64, t51074: f64, t51079: f64, t51081: f64, t51083: f64, t51086: f64, t51089: f64, t51093: f64, t51096: f64, t51099: f64, t51100: f64, t51102: f64) -> f64 {
    let t51104 = t40360 * t1561;
    let t51106 = t40864 * t14843;
    let t51110 = t14931 * t10779 * t1548 * t2724;
    let t51112 = t10811 * t14693;
    let t51114 = -0.91464571985215438874e-3_f64 * t51074 - t51079 + 0.27107389498472794075e-4_f64 * t51081 + 0.45178982497454656792e-6_f64 * t51083 + 0.54214778996945588151e-4_f64 * t51086 + 0.5421477899694558815e-4_f64 * t51089 - t51093 - t40850 + 0.45732285992607719436e-2_f64 * t40851 - t51096 + 0.15246000842785598467e-3_f64 * t40855 - t51099 - 0.12846167376791569079e-2_f64 * t51100 + 0.91464571985215438873e-3_f64 * t51102 + 0.37792653007779990369e-1_f64 * t51104 + 7.0_f64 / 4.0_f64 * t51106 - 0.30492001685571196935e-3_f64 * t51110 - 0.24009450146119052704e-1_f64 * t51112;
    t51114
}
