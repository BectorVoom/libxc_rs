//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1310/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1310(t98863: f64, t1615: f64, t27614: f64, t6176: f64, t6183: f64, t18221: f64, t28843: f64, t7978: f64, t28793: f64, t7974: f64, t98887: f64, t27556: f64, t27607: f64, t28811: f64, t28816: f64, t7968: f64, t98869: f64, t98872: f64, t98883: f64, t99079: f64) -> (f64, f64, f64) {
    let t99630 = 0.23214722222222222222e-2_f64 * t98863;
    let t99635 = t6176 * t27614 * t6183 * t1615;
    let t99639 = t7978 * t18221 * t28843;
    let t99644 = 0.61782407407407407408e-3_f64 * t28793 * t7974;
    let t99646 = 0.23214722222222222222e-2_f64 * t98887;
    let t99655 = -0.92754700520833333334e-4_f64 * t7968 * t99635 - 0.54059606481481481482e-3_f64 * t99639 - 0.17411041666666666666e-2_f64 * t98869 - 0.23214722222222222222e-2_f64 * t98872 + t99644 - 0.34822083333333333332e-2_f64 * t98883 + t99646 - 0.13901041666666666667e-2_f64 * t27607 * t28811 - 0.69505208333333333334e-3_f64 * t27607 * t28816 - 0.92754700520833333334e-4_f64 * t27556 * t28816 + 0.51015085286458333333e-3_f64 * t7968 * t99079;
    (t99630, t99635, t99655)
}
