//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 487/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk487(t221: f64, t6149: f64, t1839: f64, t476: f64, t209: f64, t1835: f64, t1195: f64, t1467: f64, t1500: f64, t4505: f64, t4544: f64, t4556: f64, t488: f64, t5571: f64, t5585: f64, t5633: f64, t5636: f64, t5677: f64, t5681: f64, t5698: f64, t6110: f64, t6114: f64, t6117: f64, t6120: f64, t6123: f64, t6125: f64, t6131: f64, t6136: f64, t6140: f64, t6145: f64) -> f64 {
    let t6150 = t221 * t6149;
    let t6153 = t1839 * t476;
    let t6155 = t221 * t6153 * t209;
    let t6158 = t1835 * t476;
    let t6160 = t221 * t6158 * t209;
    let t6163 = -0.10975822561044790898e0_f64 * t4544 * t6110 + 0.10975822561044790898e0_f64 * t1467 * t6114 - 0.25610252642437845429e0_f64 * t6117 - t5571 - t5585 - 0.54879112805223954488e-1_f64 * t488 * t6120 - 0.38415378963656768141e0_f64 * t6123 + 0.12805126321218922714e0_f64 * t6125 - 0.76830757927313536284e0_f64 * t5633 + t5636 - 0.85367508808126151427e0_f64 * t5677 + t5681 + 0.54879112805223954488e-1_f64 * t1467 * t6131 - 0.16463733841567186346e0_f64 * t5698 * t6136 + 0.16463733841567186347e0_f64 * t1467 * t6140 - 0.21341877202031537856e0_f64 * t4556 - 0.54879112805223954488e-1_f64 * t1500 * t6145 - 0.27439556402611977244e-1_f64 * t1500 * t6150 - 0.16463733841567186346e0_f64 * t4505 * t6155 + 0.54879112805223954488e-1_f64 * t1195 * t6160;
    t6163
}
