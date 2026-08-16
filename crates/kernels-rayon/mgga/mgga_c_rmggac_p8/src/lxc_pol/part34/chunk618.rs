//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 618/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk618(t15527: f64, t15281: f64, t2211: f64, t2367: f64, t1356: f64, t14451: f64, t570: f64, t5148: f64, t551: f64, t5259: f64, t558: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15528 = 0.14967802127329760705e-1_f64 * t15527;
    let t15529 = 0.14967802127329760705e-1_f64 * t15281;
    let t15530 = t2211 * t2367;
    let t15531 = t1356 * t15530;
    let t15532 = 0.39914139006212695214e-1_f64 * t15531;
    let t15533 = t14451 * t570;
    let t15534 = t5148 * t15533;
    let t15535 = 0.2993560425465952141e-1_f64 * t15534;
    let t15536 = t14451 * t551;
    let t15537 = t5259 * t15536;
    let t15538 = 0.2993560425465952141e-1_f64 * t15537;
    let t15539 = t14451 * t558;
    let t15540 = t4669 * t15539;
    (t15528, t15529, t15530, t15532, t15535, t15536, t15538, t15540)
}
