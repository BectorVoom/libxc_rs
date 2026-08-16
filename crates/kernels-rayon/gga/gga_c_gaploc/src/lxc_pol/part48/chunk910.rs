//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 910/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk910(t11849: f64, t2628: f64, t43646: f64, t43652: f64, t43657: f64, t10914: f64, t10915: f64, t1445: f64, t2684: f64, t41060: f64, t41071: f64, t43609: f64, t43611: f64, t43650: f64, t44940: f64, t45350: f64, t45415: f64, t45421: f64, t45426: f64, t45429: f64, t45432: f64, t45438: f64, t45440: f64, t7427: f64, t7573: f64, t7585: f64, t833: f64) -> f64 {
    let t45441 = t11849 * t2628;
    let t45442 = 0.29792074959875355558e-1_f64 * t45441;
    let t45451 = 0.17875244975925213335e0_f64 * t43646;
    let t45453 = 0.30674340763136599741e1_f64 * t43652;
    let t45454 = 0.20449560508757733161e1_f64 * t43657;
    let t45455 = t45415 + 0.2556195063594716645e0_f64 * t41060 - 0.2556195063594716645e0_f64 * t41071 + 0.11502877786176224903e2_f64 * t833 * t1445 * t44940 - 0.44688112439813033338e-1_f64 * t45421 + t45426 - t45429 - t45432 - 0.42900587942220512004e1_f64 * t10914 * t10915 * t45350 - t45438 + t45440 - t45442 + 0.87421871174939309263e2_f64 * t2684 * t7585 * t45350 - 0.12423108009070322895e3_f64 * t7427 * t7573 * t45350 + 0.76685851907841499353e0_f64 * t43609 + 0.76685851907841499353e0_f64 * t43611 + t45451 + 0.38342925953920749677e1_f64 * t43650 + t45453 + t45454;
    t45455
}
